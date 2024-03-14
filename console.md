   Compiling ngo-rs v0.1.0 (/home/pich/contract/ngo-rs)
error[E0609]: no field `0` on type `NearToken`
  --> src/lib.rs:98:75
   |
98 |         Promise::new(project.creator_id.clone()).transfer(total_donations.0); // Assuming NearToken has a field .0 representing the amount
   |                                                                           ^ unknown field

error[E0423]: expected function, tuple struct or tuple variant, found struct `NearToken`
   --> src/lib.rs:103:21
    |
103 |             .map_or(NearToken(0), |donations| donations.iter().map(|donation| donation.amount.0).sum::<u128>().into())
    |                     ^^^^^^^^^^^^ help: use struct literal syntax instead: `NearToken { inner: val }`
    |
   ::: /home/pich/.cargo/registry/src/index.crates.io-6f17d22bba15001f/near-token-0.2.0/src/lib.rs:44:1
    |
44  | pub struct NearToken {
    | -------------------- `NearToken` defined here

error[E0609]: no field `0` on type `NearToken`
   --> src/lib.rs:103:95
    |
103 |             .map_or(NearToken(0), |donations| donations.iter().map(|donation| donation.amount.0).sum::<u128>().into())
    |                                                                                               ^ unknown field

Some errors have detailed explanations: E0423, E0609.
For more information about an error, try `rustc --explain E0423`.
error: could not compile `ngo-rs` (lib) due to 3 previous errors
