pub fn claim_funds(&mut self, project_id: String) {
    let project = self.projects.get(&project_id).expect("Project not found");

    if env::block_timestamp() <= project.end_date {
        env::panic_str("Project has not ended yet");
    }

    if project.funds_claimed {
        env::panic_str("Funds have already been claimed");
    }

    let total_donations: u128 = self.donations.remove(&project_id).unwrap_or_default()
        .into_iter()
        .map(|donation| {
            let NearToken(amount) = donation.amount; // Assuming NearToken is a tuple struct around u128
            amount
        })
        .sum();

    let total_donations_near_token = NearToken(total_donations); // Convert back to NearToken

    Promise::new(project.creator_id.clone()).transfer(total_donations_near_token);
    self.projects.get_mut(&project_id).unwrap().funds_claimed = true;
}
