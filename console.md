   Compiling ngo-rs v0.1.0 (/home/pich/contract/ngo-rs)
error[E0308]: mismatched types
   --> src/lib.rs:109:59
    |
109 |         Promise::new(project.creator_id.clone()).transfer(total_donations); // Transfer the summed u128 value
    |                                                  -------- ^^^^^^^^^^^^^^^ expected `NearToken`, found `u128`
    |                                                  |
    |                                                  arguments to this method are incorrect
    |
note: method defined here
   --> /home/pich/.cargo/registry/src/index.crates.io-6f17d22bba15001f/near-sdk-5.0.0/src/promise.rs:317:12
    |
317 |     pub fn transfer(self, amount: NearToken) -> Self {
    |            ^^^^^^^^

For more information about this error, try `rustc --explain E0308`.
error: could not compile `ngo-rs` (lib) due to previous error
