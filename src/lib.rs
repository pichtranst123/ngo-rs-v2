use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Timestamp, PanicOnDefault, Promise, payable};
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use near_token::NearToken;

const ONE_HOUR_IN_NANOSECONDS: Timestamp = 60 * 60 * 1_000_000_000;

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
        self.projects.insert(project_name, project_metadata);
    }

    #[payable]
    pub fn create_donation(&mut self, project_id: String) {
        let donor_id = env::signer_account_id();
        let donation_amount = NearToken(env::attached_deposit()); // Wrap the attached deposit in NearToken

        assert!(self.projects.contains_key(&project_id), "Project not found");

        let donation = Donation {
            donor_id,
            amount: donation_amount,
            donation_time: env::block_timestamp(),
        };
        self.donations.entry(project_id).or_insert_with(Vec::new).push(donation);
    }

    pub fn claim_funds(&mut self, project_id: String) {
        let project = self.projects.get_mut(&project_id).expect("Project not found");
        assert!(!project.funds_claimed, "Funds have already been claimed");
        assert!(env::block_timestamp() > project.end_date, "Project has not ended yet");

        let total_donations: NearToken = self.get_total_donations_for_project(&project_id);
        assert!(total_donations <= project.target_amount, "Total donations exceed target amount");

        project.funds_claimed = true;
        Promise::new(project.creator_id.clone()).transfer(total_donations.0); // Assuming NearToken has a field .0 representing the amount
    }

    pub fn get_total_donations_for_project(&self, project_id: &String) -> NearToken {
        self.donations.get(project_id)
            .map_or(NearToken(0), |donations| donations.iter().map(|donation| donation.amount.0).sum::<u128>().into())
    }

    // Additional functions like getters can be added below
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
