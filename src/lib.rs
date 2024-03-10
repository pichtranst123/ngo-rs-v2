use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Timestamp, PanicOnDefault};
use near_sdk::json_types::U128;
use near_sdk::collections::HashMap;
use near_sdk::serde::{Deserialize, Serialize};

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
    target_amount: U128,
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

    pub fn create_project(&mut self, project_name: String, project_description: String, target_amount: String, ipfs_image: String, ipfs_hash: Vec<String>, end_date: Timestamp) {
        let target_amount: U128 = target_amount.parse().expect("Invalid target amount");
        let creator_id = env::signer_account_id();
        let start_date = env::block_timestamp();
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
        };
        self.projects.insert(project_id, project_metadata);
    }

    pub fn create_donation(&mut self, project_id: String, amount: U128) {
        let donor_id = env::signer_account_id();
        let donation = Donation {
            donor_id,
            amount,
            donation_time: env::block_timestamp(),
        };
        let project_metadata = self.projects.get_mut(&project_id).expect("Project not found");
        project_metadata.current_amount = U128(project_metadata.current_amount.0 + amount.0);
        self.donations.entry(project_id).or_insert_with(Vec::new).push(donation);
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

    pub fn get_donations_on_going(&self) -> Vec<(String, ProjectMetadata)> {
        let current_time = env::block_timestamp();
        self.projects.iter().filter(|(_, project)| project.end_date > current_time).map(|(id, project)| (id.clone(), project.clone())).collect()
    }

    pub fn get_donations_ended(&self) -> Vec<(String, ProjectMetadata)> {
        let current_time = env::block_timestamp();
        self.projects.iter().filter(|(_, project)| project.end_date <= current_time).map(|(id, project)| (id.clone(), project.clone())).collect()
    }
}
