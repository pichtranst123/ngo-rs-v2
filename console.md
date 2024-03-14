error[E0308]: mismatched types
  --> src/lib.rs:77:64
   |
77 |     let donation_amount: NearToken = NearToken::from_yoctonear(env::attached_deposit());
   |                                      ------------------------- ^^^^^^^^^^^^^^^^^^^^^^^ expected `u128`, found `NearToken`
   |                                      |
   |                                      arguments to this function are incorrect
   |
note: associated function defined here
  --> /home/pich/.cargo/registry/src/index.crates.io-6f17d22bba15001f/near-token-0.2.0/src/lib.rs:58:18
   |
58 |     pub const fn from_yoctonear(inner: u128) -> Self {
   |                  ^^^^^^^^^^^^^^

error[E0369]: cannot add `NearToken` to `NearToken`
  --> src/lib.rs:86:55
   |
86 |     let new_total_donations = current_total_donations + donation_amount;
   |                               ----------------------- ^ --------------- NearToken
   |                               |
   |                               NearToken

error[E0599]: no method named `to_yoctonear` found for struct `NearToken` in the current scope
   --> src/lib.rs:113:75
    |
113 |         Promise::new(project.creator_id.clone()).transfer(total_donations.to_yoctonear());
    |                                                                           ^^^^^^^^^^^^ help: there is a method with a similar name: `as_yoctonear`

error[E0599]: no method named `to_yoctonear` found for struct `NearToken` in the current scope
   --> src/lib.rs:118:103
    |
118 |     .map_or(NearToken::from_yoctonear(0), |donations| donations.iter().map(|donation| donation.amount.to_yoctonear()).sum::<u128>().into())
    |                                                                                                       ^^^^^^^^^^^^ help: there is a method with a similar name: `as_yoctonear`

Some errors have detailed explanations: E0308, E0369, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `ngo-rs` (lib) due to 4 previous errors
