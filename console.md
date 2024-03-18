warning: unused import: `near_token::NearToken`
 --> src/lib.rs:5:5
  |
5 | use near_token::NearToken;
  |     ^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

error[E0599]: no method named `value` found for struct `NearToken` in the current scope
  --> src/lib.rs:81:42
   |
81 |     let deposit_value = attached_deposit.value(); // Convert NearToken to u128
   |                                          ^^^^^ method not found in `NearToken`

error[E0308]: mismatched types
   --> src/lib.rs:108:55
    |
108 |     Promise::new(project.creator_id.clone()).transfer(project.collected_amount) // Assuming collected_amount is u128
    |                                              -------- ^^^^^^^^^^^^^^^^^^^^^^^^ expected `NearToken`, found `u128`
    |                                              |
    |                                              arguments to this method are incorrect
    |
