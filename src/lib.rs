use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use std::collections::HashMap;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Promise, Timestamp};
use near_sdk::serde::{Deserialize, Serialize};
use near_token::NearToken;
const ONE_YOCTO: u128 = 1;

const ONE_HOUR_IN_NANOSECONDS: Timestamp = 60 * 60 * 1_000_000_000;
const ONE_DAY_IN_NANOSECONDS: Timestamp = 24 * ONE_HOUR_IN_NANOSECONDS;
const SEVEN_DAYS_IN_NANOSECONDS: Timestamp = 7 * ONE_DAY_IN_NANOSECONDS;
const THIRTY_DAYS_IN_NANOSECONDS: Timestamp = 30 * ONE_DAY_IN_NANOSECONDS;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct DonationContract {
    projects: HashMap<String, Project>,
    donations: HashMap<String, Vec<Donation>>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
pub struct Project {
    creator_id: AccountId,
    project_name: String,
    project_description: String,
    target_amount: u128,
    collected_amount: u128, 
    ipfs_image: String,
    ipfs_hash: Vec<String>,
    start_date: Timestamp,
    end_date: Timestamp,
    funds_claimed: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
pub struct Donation {
    donor_id: AccountId,
    amount: u128,
    donation_time: Timestamp,
}

#[near_bindgen]
impl DonationContract {
    #[init]
    pub fn new() -> Self {
        Self {
            projects: HashMap::new(),
            donations: HashMap::new(),
        }
    }

    pub fn create_project(&mut self, project_name: String, project_description: String, target_amount: u128, ipfs_image: String, ipfs_hash: Vec<String>, duration: u8) {
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
            collected_amount: 0,
            ipfs_image,
            ipfs_hash,
            start_date,
            end_date,
            funds_claimed: false,
        };
        self.projects.insert(project_name, project_metadata);
    }

#[payable]
pub fn create_donate(&mut self, project_id: String) {
        let donor_id = env::signer_account_id();
        let attached_deposit = env::attached_deposit();
        assert!(attached_deposit > 0, "Donation must be greater than 0");

        let donation = Donation {
            donor_id,
            amount: attached_deposit,
            donation_time: env::block_timestamp(),
        };

        let mut donations = self.donations.get(&project_id).unwrap_or_else(Vec::new);
        donations.push(donation);
        self.donations.insert(&project_id, &donations);
    }

pub fn claim_funds(&mut self, project_id: String) -> Promise {
    let account_id = env::signer_account_id();
    let project = self.projects.get_mut(&project_id).expect("Project not found");

    assert_eq!(account_id, project.creator_id, "Only the project creator can claim funds");
    assert!(env::block_timestamp() > project.end_date, "Project is still active");
    assert!(!project.funds_claimed, "Funds have already been claimed");
    assert!(project.collected_amount >= project.target_amount, "Target amount not reached");

    project.funds_claimed = true;

    // Transfer collected_amount as is, since it's a u128 value and compatible with transfer method
    Promise::new(project.creator_id.clone()).transfer(project.collected_amount)
}
    
    pub fn get_projects(&self) -> Vec<(String, Project)> {
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
}
