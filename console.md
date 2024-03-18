#[payable]
pub fn create_donate(&mut self, project_id: String, amount: String) {
    let attached_deposit = env::attached_deposit();
    
    // Assume amount is parsed as a u128 for comparison
    let donation_amount_parsed: u128 = amount.parse().expect("Invalid amount format");
    assert_eq!(attached_deposit, donation_amount_parsed, "Attached deposit must match the specified amount");
    
    let project = self.projects.get(&project_id).expect("Project not found");

    // Use the correct constructor or conversion method for NearToken
    let donation_amount = NearToken::from_u128(donation_amount_parsed); // Adjust this line as needed

    let donation = Donation {
        donor_id: env::signer_account_id(),
        amount: donation_amount, // Ensure this matches the NearToken type
        donation_time: env::block_timestamp(),
    };

    let mut donations = self.donations.get(&project_id).cloned().unwrap_or_default();
    donations.push(donation);
    self.donations.insert(project_id.clone(), donations);

    // Transfer the attached deposit (donation) to the project creator
    Promise::new(project.creator_id.clone()).transfer(attached_deposit);
}
