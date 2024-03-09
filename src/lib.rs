use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::U128;
use std::collections::HashMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DonationProject {
    projects: Vec<ProjectMetadata>,
    project_index: HashMap<String, usize>, // Maps project_id to index in `projects`
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ProjectMetadata {
    project_id: String,
    creator_id: AccountId,
    project_name: String,
    project_description: String,
    target_amount: U128,
    current_amount: U128,
    start_date: u64,
    end_date: u64,
}

#[near_bindgen]
impl DonationProject {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized.");
        Self {
            projects: Vec::new(),
            project_index: HashMap::new(),
        }
    }

    pub fn create_project(&mut self, project_id: String, creator_id: AccountId, project_name: String, project_description: String, target_amount: U128, start_date: u64, end_date: u64) {
        let project_metadata = ProjectMetadata {
            project_id: project_id.clone(),
            creator_id,
            project_name,
            project_description,
            target_amount,
            current_amount: U128(0),
            start_date,
            end_date,
        };
        self.projects.push(project_metadata);
        let project_index = self.projects.len() - 1;
        self.project_index.insert(project_id, project_index);
    }


    pub fn create_donation(&mut self, project_id: String, donor_id: AccountId, amount: U128) {
        let mut project = self.projects.get(&project_id).expect("Project not found");
        project.current_amount = U128(project.current_amount.0 + amount.0);
        self.projects.insert(&project_id, &project);

        let donation = Donation {
            donor_id,
            amount,
            donation_time: env::block_timestamp(),
        };
        let mut project_donations = self.donations.get(&project_id).unwrap_or_else(|| Vector::new(b"v".to_vec()));
        project_donations.push(&donation);
        self.donations.insert(&project_id, &project_donations);
    }

    pub fn get_projects(&self) -> Vec<(String, ProjectMetadata)> {
        self.projects.iter().collect()
    }

    pub fn get_donors_by_project_id(&self, project_id: String) -> Vec<Donation> {
        self.donations.get(&project_id).unwrap_or_else(|| Vector::new(b"v".to_vec())).to_vec()
    }

    // This function iterates over all projects and donations, which might be inefficient for large datasets.
    pub fn get_donations_by_donor_id(&self, donor_id: AccountId) -> Vec<(String, Donation)> {
        let mut results: Vec<(String, Donation)> = Vec::new();
        for (project_id, donations) in self.donations.iter() {
            for donation in donations.iter() {
                if donation.donor_id == donor_id {
                    results.push((project_id.clone(), donation.clone()));
                }
            }
        }
        results
    }
}
