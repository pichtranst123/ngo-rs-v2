use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use std::collections::HashMap;
use std::fmt::format;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Promise, Timestamp};
use near_sdk::serde::{Deserialize, Serialize};
use near_token::NearToken;
const ONE_YOCTO: u128 = 1;
use std::string::String;

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

        let project_id = format!("{}_{}",creator_id,start_date);
        let project_metadata = Project {
            creator_id,
            project_name: project_name.clone(),
            project_description,
            target_amount,
            ipfs_image,
            ipfs_hash,
            start_date,
            end_date,
            funds_claimed: false,
        };
        self.projects.insert(project_id, project_metadata);
    }

    #[payable]
    pub fn create_donate(&mut self, project_id: String) {
        let project = self.projects.get(&project_id).expect("Project not found");
        let attached_deposit = env::attached_deposit();
    
        let donation = Donation {
            donor_id: env::signer_account_id(),
            amount: attached_deposit, // Ensure NearToken can be directly assigned from u128
            donation_time: env::block_timestamp(),
        };
    
        let mut donations = self.donations.get(&project_id).cloned().unwrap_or_default();
        donations.push(donation);
        self.donations.insert(project_id, donations);
    
        Promise::new(project.creator_id.clone()).transfer(attached_deposit);
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
