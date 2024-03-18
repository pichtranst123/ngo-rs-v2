#[payable]
pub fn create_donate(&mut self, project_id: String, amount: NearToken) {
    // Get the attached deposit, which is in yoctoNEAR
    let attached_deposit = env::attached_deposit();

    // Ensure the attached deposit matches the amount specified in the function call
    assert_eq!(attached_deposit, amount.into(), "Attached deposit must match the specified amount");

    let project = self.projects.get(&project_id).expect("Project not found");

    let donation = Donation {
        donor_id: env::signer_account_id(),
        amount: amount, // Directly use the NearToken amount specified in the function call
        donation_time: env::block_timestamp(),
    };

    let mut donations = self.donations.get(&project_id).cloned().unwrap_or_default();
    donations.push(donation);
    self.donations.insert(project_id, donations);

    // Optionally, transfer the attached deposit to the project creator or handle it as needed
    Promise::new(project.creator_id.clone()).transfer(attached_deposit);
}
