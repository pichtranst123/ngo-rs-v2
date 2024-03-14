use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Timestamp, PanicOnDefault, Promise};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
#[serde(crate = "near_sdk::serde")]
pub struct ProjectMetadata {
    creator_id: AccountId,
    project_name: String,
    project_description: String,
    target_amount: u128,
    current_amount: U128,
    ipfs_image: String,
    ipfs_hash: Vec<String>,
    start_date: Timestamp,
    end_date: Timestamp,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Donation {
    donor_id: AccountId,
    amount: U128,
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

  pub fn create_project(&mut self, project_name: String, project_description: String, target_amount: u128, ipfs_image: String, ipfs_hash: Vec<String>, duration: u8) {
        let creator_id = env::signer_account_id();
        let start_date = env::block_timestamp();
        let end_date = match duration {
            0 => start_date + 4 * ONE_HOUR_IN_NANOSECONDS,
            1 => start_date + ONE_DAY_IN_NANOSECONDS,
            2 => start_date + SEVEN_DAYS_IN_NANOSECONDS,
            3 => start_date + THIRTY_DAYS_IN_NANOSECONDS,
            _ => env::panic_str("Invalid duration."),
        };

        let project_id = format!("{}_{}_{}", creator_id, start_date, env::block_index());
        let project_metadata = ProjectMetadata {
            creator_id,
            project_name,
            project_description,
            target_amount,
            current_amount: U128(0),
            ipfs_image,
            ipfs_hash,
            start_date,
            end_date,
            funds_claimed: false,
        };
        self.projects.insert(project_id, project_metadata);
    }

    pub fn create_donation(&mut self, project_id: String, amount: U128) {
        let donor_id = env::signer_account_id();
        let donation_time = env::block_timestamp();
        let donation = Donation {
            donor_id,
            amount,
            donation_time,
        };
        let project_metadata = self.projects.get_mut(&project_id).expect("Project not found");
        project_metadata.current_amount = U128((project_metadata.current_amount.0 + amount.0) as u128);
        self.donations.entry(project_id).or_insert_with(Vec::new).push(donation);
    }

    pub fn claim_funds(&mut self, project_id: String) {
        let project = self.projects.get_mut(&project_id).expect("Project not found");
        let current_time = env::block_timestamp();
        let caller_id = env::predecessor_account_id();

        if caller_id != project.creator_id {
            env::panic_str("Only the project creator can claim the funds");
        }
        if current_time < project.end_date {
            env::panic_str("Project has not ended yet");
        }
        if project.funds_claimed {
            env::panic_str("Funds have already been claimed");
        }

        let amount_to_transfer = project.current_amount.0;
        project.current_amount = U128(0);
        project.funds_claimed = true;

        Promise::new(project.creator_id.clone()).transfer(amount_to_transfer);
    }

    pub fn get_projects(&self) -> Vec<(String, ProjectMetadata)> {
        self.projects.iter().map(|(id, project)| (id.clone(), project.clone())).collect()
    }

    pub fn get_donors_by_project_id(&self, project_id: String) -> Vec<Donation> {
        self.donations.get(&project_id).cloned().unwrap_or_else(Vec::new)
    }

    pub fn get_donations_by_donor_id(&self, donor_id: AccountId) -> Vec<(String, String)> {
        self.donations.iter().filter_map(|(project_id, donations)| {
            let project_name = self.projects.get(project_id).map(|p| p.project_name.clone()).unwrap_or_default();
            donations.iter().find(|donation| donation.donor_id == donor_id).map(|_| (project_id.clone(), project_name))
        }).collect()
    }

    pub fn get_donations_ongoing(&self) -> Vec<(String, ProjectMetadata)> {
        let current_time = env::block_timestamp();
        self.projects.iter().filter(|(_, project)| project.end_date > current_time).map(|(id, project)| (id.clone(), project.clone())).collect()
    }

    pub fn get_donations_ended(&self) -> Vec<(String, ProjectMetadata)> {
        let current_time = env::block_timestamp();
        self.projects.iter().filter(|(_, project)| project.end_date <= current_time).map(|(id, project)| (id.clone(), project.clone())).collect()
    }
}
