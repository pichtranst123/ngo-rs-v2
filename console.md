#[payable]
pub fn create_donate(&mut self, project_id: String, amount: String) {
    let attached_deposit = env::attached_deposit();
    
    // Parse the amount string into NearToken, adjust this part according to your NearToken implementation
    let donation_amount = NearToken::from_str(&amount).expect("Invalid amount format");

    assert_eq!(attached_deposit, donation_amount.into(), "Attached deposit must match the specified amount");

    let project = self.projects.get(&project_id).expect("Project not found");
    
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
