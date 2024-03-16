use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Promise, Timestamp};
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use near_token::NearToken;

const ONE_HOUR_IN_NANOSECONDS: Timestamp = 60 * 60 * 1_000_000_000;
const ONE_DAY_IN_NANOSECONDS: Timestamp = 24 * ONE_HOUR_IN_NANOSECONDS;
const SEVEN_DAYS_IN_NANOSECONDS: Timestamp = 7 * ONE_DAY_IN_NANOSECONDS;
const THIRTY_DAYS_IN_NANOSECONDS: Timestamp = 30 * ONE_DAY_IN_NANOSECONDS;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct DonationProject {
    projects: HashMap<String, ProjectMetadata>,
    donations: HashMap<String, Vec<Donation>>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
pub struct ProjectMetadata {
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
impl DonationProject {
    #[init]
    pub fn new() -> Self {
        Self {
            projects: HashMap::new(),
            donations: HashMap::new(),
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

        let project_metadata = ProjectMetadata {
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
    pub fn create_donation(&mut self, project_id: String, donate_amount: NearToken) {
        let attached_deposit = NearToken::from_yoctonear(env::attached_deposit());
        if donate_amount != attached_deposit {
            env::panic_str("Attached deposit must match the specified donation amount");
        }

        let donor_id = env::signer_account_id();

        if !self.projects.contains_key(&project_id) {
            env::panic_str("Project not found");
        }

        let donation = Donation {
            donor_id,
            amount: donate_amount,
            donation_time: env::block_timestamp(),
        };

        self.donations.entry(project_id).or_insert_with(Vec::new).push(donation);
    }

         pub fn claim_funds(&mut self, project_id: String) {
        let project = self.projects.get(&project_id).expect("Project not found");

        if env::block_timestamp() <= project.end_date {
            env::panic_str("Project has not ended yet");
        }

        if project.funds_claimed {
            env::panic_str("Funds have already been claimed");
        }

        let total_donations = self.donations.remove(&project_id).unwrap_or_default()
            .into_iter()
            .map(|donation| donation.amount)
            .reduce(|acc, donation| acc + donation)
            .unwrap_or(NearToken::from_yoctonear(0));

        Promise::new(project.creator_id.clone()).transfer(total_donations.into());
        self.projects.get_mut(&project_id).unwrap().funds_claimed = true;
    }
    
fn get_total_donations_for_project(&self, project_id: &String) -> NearToken {
    self.donations.get(project_id)
        .map_or(NearToken::from_yocto(0), |donations| donations.iter().map(|donation| donation.amount).sum())
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
}
