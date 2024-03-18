use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Promise, PromiseOrValue,Timestamp};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::Gas;

const BASE_GAS: Gas = 25_000_000_000_000;
const ONE_YOCTO: u128 = 1;

const ONE_HOUR_IN_NANOSECONDS: Timestamp = 60 * 60 * 1_000_000_000;
const ONE_DAY_IN_NANOSECONDS: Timestamp = 24 * ONE_HOUR_IN_NANOSECONDS;
const SEVEN_DAYS_IN_NANOSECONDS: Timestamp = 7 * ONE_DAY_IN_NANOSECONDS;
const THIRTY_DAYS_IN_NANOSECONDS: Timestamp = 30 * ONE_DAY_IN_NANOSECONDS;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct DonationContract {
    projects: LookupMap<String, Project>,
    donations: UnorderedMap<String, Vec<Donation>>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
pub struct Project {
    creator_id: AccountId,
    project_name: String,
    project_description: String,
    target_amount: NearToken,
    ipfs_image: String,
    ipfs_hash: Vec<String>,
    start_date: Timestamp,
    end_date: Timestamp,
    funds_claimed: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
pub struct Donation {
    donor_id: AccountId,
    amount: NearToken,
    donation_time: Timestamp,
}

#[near_bindgen]
impl DonationContract {
    #[init]
    pub fn new() -> Self {
        Self {
            projects: LookupMap::new(b"p"),
            donations: UnorderedMap::new(b"d"),
        }
    }

    pub fn create_project(&mut self, project_name: String, project_description: String, target_amount: NearToken, ipfs_image: String, ipfs_hash: Vec<String>, duration: u8) {
        let creator_id = env::signer_account_id();
        let start_date = env::block_timestamp();
        let end_date = start_date + match duration {
            0 => 4 * ONE_HOUR_IN_NANOSECONDS,
            1 => ONE_DAY_IN_NANOSECONDS,
            2 => SEVEN_DAYS_IN_NANOSECONDS,
            3 => THIRTY_DAYS_IN_NANOSECONDS,
            _ => env::panic_str("Invalid duration."),
        };

        let project_metadata = Project {
            creator_id,
            project_name,
            project_description,
            target_amount,
            ipfs_image,
            ipfs_hash,
            start_date,
            end_date,
            funds_claimed: false,
        };
        self.projects.insert(project_name.clone(), project_metadata);
    }

    #[payable]
    pub fn donate(&mut self, project_id: String) {
        let donor_id = env::signer_account_id();
        let attached_deposit = env::attached_deposit();
        assert!(attached_deposit > 0, "Donation must be greater than 0");

        let mut project = self.projects.get(&project_id).expect("Project not found");
        assert!(env::block_timestamp() < project.end_date, "Project has ended");

        let new_collected_amount = project.target_amount.0 + attached_deposit;
        project.target_amount = U128(new_collected_amount);

        self.projects.insert(&project_id, &project);

        let donation = Donation {
            donor_id,
            amount: U128(attached_deposit),
            donation_time: env::block_timestamp(),
        };
        self.donations.entry(project_id).or_insert_with(Vec::new).push(donation);
    }

    pub fn claim_funds(&mut self, project_id: String) -> Promise {
        let account_id = env::signer_account_id();
        let mut project = self.projects.get(&project_id).expect("Project not found");

        assert_eq!(account_id, project.creator_id, "Only the project creator can claim funds");
        assert!(env::block_timestamp() > project.end_date, "Project is still active");
        assert!(!project.funds_claimed, "Funds have already been claimed");
        assert!(project.target_amount.0 >= project.target_amount.0, "Target amount not reached");

        project.funds_claimed = true;
        self.projects.insert(&project_id, &project);

        Promise::new(project.creator_id.clone()).transfer(project.target_amount.0)
    }
    pub fn get_projects(&self) -> Vec<(String, ProjectMetadata)> {
        self.projects.iter().map(|(id, project)| (id.clone(), project.clone())).collect()
    }

    pub fn get_donors_by_project_id(&self, project_id: String) -> Vec<Donation> {
        self.donations.get(&project_id).cloned().unwrap_or_else(Vec::new)
    }

    pub fn get_donations_by_donor_id(&self, donor_id: AccountId) -> Vec<(String, Donation)> {
        self.donations.iter().filter_map(|(project_id, donations)| {
            donations.iter().find(|donation| donation.donor_id == donor_id).map(|donation| (project_id.clone(), donation.clone()))
        }).collect()
    }
     pub fn get_projects_claimed(&self) -> Vec<String> {
        self.projects.iter()
            .filter_map(|(project_id, project)| {
                if project.funds_claimed {
                    Some(project_id)
                } else {
                    None
                }
            })
            .collect()
    }
      pub fn get_projects_not_claimed(&self) -> Vec<String> {
        self.projects.iter()
            .filter_map(|(project_id, project)| {
                if !project.funds_claimed && env::block_timestamp() > project.end_date && project.target_amount.0 <= project.target_amount.0 {
                    Some(project_id)
                } else {
                    None
                }
            })
            .collect()
    }
}
