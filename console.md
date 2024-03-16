error[E0532]: expected tuple struct or tuple variant, found struct `NearToken`
   --> src/lib.rs:107:21
    |
107 |                 let NearToken(amount) = donation.amount; // Assuming NearToken is a tuple struct around u128
    |                     ^^^^^^^^^^^^^^^^^ help: use struct pattern syntax instead: `NearToken { inner: amount }`
    |
   ::: /home/pich/.cargo/registry/src/index.crates.io-6f17d22bba15001f/near-token-0.2.0/src/lib.rs:44:1
    |
44  | pub struct NearToken {
    | -------------------- `NearToken` defined here

error[E0423]: expected function, tuple struct or tuple variant, found struct `NearToken`
   --> src/lib.rs:112:42
    |
112 |         let total_donations_near_token = NearToken(total_donations); // Convert back to NearToken
    |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use struct literal syntax instead: `NearToken { inner: val }`
    |
   ::: /home/pich/.cargo/registry/src/index.crates.io-6f17d22bba15001f/near-token-0.2.0/src/lib.rs:44:1
    |
44  | pub struct NearToken {
    | -------------------- `NearToken` defined here

Some errors have detailed explanations: E0423, E0532.
For more information about an error, try `rustc --explain E0423`.
error: could not compile `ngo-rs` (lib) due to 2 previous errors
