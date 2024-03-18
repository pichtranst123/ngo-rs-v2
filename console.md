   #[payable]
    pub fn create_donate(&mut self, project_id: String, amount: String) {
        let attached_deposit = env::attached_deposit();
        
        // Parse the amount string into u128
        let donation_amount_u128: NearToken = amount.parse().expect("Invalid amount format");
        assert_eq!(attached_deposit, donation_amount_u128, "Attached deposit must match the specified amount");
        
        let project = self.projects.get(&project_id).expect("Project not found");
        
        // Convert donation_amount_u128 to NearToken
        // Adjust this part according to your NearToken's implementation
        let donation_amount = NearToken::new(donation_amount_u128);

        let donation = Donation {
            donor_id: env::signer_account_id(),
            amount: donation_amount,
            donation_time: env::block_timestamp(),
        };

        let mut donations = self.donations.get(&project_id).cloned().unwrap_or_default();
        donations.push(donation);
        self.donations.insert(project_id.clone(), donations);

        // Transfer the attached deposit (donation) to the project creator
        Promise::new(project.creator_id.clone()).transfer(attached_deposit);
    }
