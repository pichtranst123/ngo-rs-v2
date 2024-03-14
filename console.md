error[E0599]: no function or associated item named `new` found for struct `NearToken` in the current scope
  --> src/lib.rs:77:49
   |
77 |     let donation_amount: NearToken = NearToken::new(env::attached_deposit()); // Wrap the attached deposit in NearToken
   |                                                 ^^^ function or associated item not found in `NearToken`

error[E0599]: no method named `value` found for struct `NearToken` in the current scope
   --> src/lib.rs:113:75
    |
113 |         Promise::new(project.creator_id.clone()).transfer(total_donations.value()); // Assuming .value() method to get the inner u128 value
    |                                                                           ^^^^^ method not found in `NearToken`

error[E0599]: no function or associated item named `new` found for struct `NearToken` in the current scope
   --> src/lib.rs:118:32
    |
118 |             .map_or(NearToken::new(0), |donations| donations.iter().map(|donation| donation.amount.value()).sum::<u128>().into()) // Assuming NearToken::new() and .value()
    |                                ^^^ function or associated item not found in `NearToken`

error[E0599]: no method named `value` found for struct `NearToken` in the current scope
   --> src/lib.rs:118:100
    |
118 |             .map_or(NearToken::new(0), |donations| donations.iter().map(|donation| donation.amount.value()).sum::<u128>().into()) // Assuming NearToken::new() and .value()
    |                                                                                                    ^^^^^ method not found in `NearToken`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `ngo-rs` (lib) due to 4 previous errors
