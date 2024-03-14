error[E0599]: no function or associated item named `from_yocto` found for struct `NearToken` in the current scope
  --> src/lib.rs:77:49
   |
77 |     let donation_amount: NearToken = NearToken::from_yocto(env::attached_deposit());
   |                                                 ^^^^^^^^^^
   |                                                 |
   |                                                 function or associated item not found in `NearToken`
   |                                                 help: there is an associated function with a similar name: `from_yoctonear`

error[E0609]: no field `amount` on type `NearToken`
   --> src/lib.rs:113:75
    |
113 |         Promise::new(project.creator_id.clone()).transfer(total_donations.amount);
    |                                                                           ^^^^^^ unknown field

error[E0599]: no function or associated item named `from_yocto` found for struct `NearToken` in the current scope
   --> src/lib.rs:118:24
    |
118 |     .map_or(NearToken::from_yocto(0), |donations| donations.iter().map(|donation| donation.amount.amount).sum::<u128>().into())
    |                        ^^^^^^^^^^
    |                        |
    |                        function or associated item not found in `NearToken`
    |                        help: there is an associated function with a similar name: `from_yoctonear`

error[E0609]: no field `amount` on type `NearToken`
   --> src/lib.rs:118:99
    |
118 |     .map_or(NearToken::from_yocto(0), |donations| donations.iter().map(|donation| donation.amount.amount).sum::<u128>().into())
    |                                                                                                   ^^^^^^ unknown field

Some errors have detailed explanations: E0599, E0609.
For more information about an error, try `rustc --explain E0599`.
error: could not compile `ngo-rs` (lib) due to 4 previous errors
