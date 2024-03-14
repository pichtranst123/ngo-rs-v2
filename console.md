warning: use of deprecated function `near_sdk::env::block_index`: Use block_height instead
  --> src/lib.rs:64:75
   |
64 |         let project_id = format!("{}_{}_{}", creator_id, start_date, env::block_index());
   |                                                                           ^^^^^^^^^^^
   |
   = note: `#[warn(deprecated)]` on by default

error[E0308]: mismatched types
   --> src/lib.rs:112:59
    |
112 |         Promise::new(project.creator_id.clone()).transfer(amount_to_transfer);
    |                                                  -------- ^^^^^^^^^^^^^^^^^^ expected `NearToken`, found `u128`
    |                                                  |
    |                                                  arguments to this method are incorrect
    |
note: method defined here
   --> /home/pich/.cargo/registry/src/index.crates.io-6f17d22bba15001f/near-sdk-5.0.0/src/promise.rs:317:12
    |
317 |     pub fn transfer(self, amount: NearToken) -> Self {
    |            ^^^^^^^^

For more information about this error, try `rustc --explain E0308`.
warning: `ngo-rs` (lib) generated 1 warning
error: could not compile `ngo-rs` (lib) due to previous error; 1 warning emitted
