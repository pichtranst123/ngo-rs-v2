use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Timestamp};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::from_str;
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DonationProject {
    projects: Vec<(String, ProjectMetadata)>, // Using a Vec to store project IDs paired with their metadata
    donations: Vec<(String, Vec<Donation>)>, // Using a Vec to store project IDs paired with a Vec of donations
    project_counter: u64,

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
    donation_time: u64,
}

impl Default for DonationProject {
    fn default() -> Self {
        Self::new()
    }
}


#[near_bindgen]
impl DonationProject {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized.");
        Self {
            projects: Vec::new(),
            donations: Vec::new(),
            project_counter: 0,
        }
    }
    pub fn create_project(&mut self, project_name: String, project_description: String, target_amount: String, ipfs_image: String, ipfs_hash: Vec<String>, end_date: Timestamp) {        
        let target_amount: U128 = from_str(&target_amount).expect("Invalid target amount");
        let creator_id = env::signer_account_id();
        let start_date = env::block_timestamp();
        self.project_counter += 1;
        let project_id = format!("{}_{}_{}", creator_id, start_date, self.project_counter);
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
        // Ensure the project ID is unique
        assert!(self.projects.iter().all(|(id, _)| id != &project_id), "Project ID already exists");
        self.projects.push((project_id, project_metadata));
    }
    

    pub fn create_donation(&mut self, project_id: String, amount: U128) {
        let donor_id = env::signer_account_id();
        let project_index = self.projects.iter().position(|(id, _)| id == &project_id).expect("Project not found");
        let (_, project_metadata) = &mut self.projects[project_index];
        project_metadata.current_amount = U128(project_metadata.current_amount.0 + amount.0);

        let donation = Donation {
            donor_id,
            amount,
            donation_time: env::block_timestamp(),
        };

        match self.donations.iter_mut().find(|(id, _)| id == &project_id) {
            Some((_, donations_vec)) => donations_vec.push(donation),
            None => self.donations.push((project_id, vec![donation])),
        };
    }

    pub fn get_projects(&self) -> Vec<(String, ProjectMetadata)> {
        self.projects.clone()
    }

    pub fn get_donors_by_project_id(&self, project_id: String) -> Vec<Donation> {
        self.donations.iter()
            .find(|(id, _)| id == &project_id)
            .map_or_else(Vec::new, |(_, donations)| donations.clone())
    }

    pub fn get_donations_by_donor_id(&self, donor_id: AccountId) -> Vec<(String, Donation)> {
        self.donations.iter()
            .flat_map(|(project_id, donations)| {
                donations.iter()
                    .filter(|donation| donation.donor_id == donor_id)
                    .map(move |donation| (project_id.clone(), donation.clone()))
            })
            .collect()
    }
}
