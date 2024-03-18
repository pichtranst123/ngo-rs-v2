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

    
