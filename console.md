error: proc-macro derive panicked
 --> src/lib.rs:8:10
  |
8 | #[derive(BorshDeserialize, BorshSerialize)]
  |          ^^^^^^^^^^^^^^^^
  |
  = help: message: called `Result::unwrap()` on an `Err` value: Could not find `borsh` in `dependencies` or `dev-dependencies` in `/home/pich/contract/ngo-rs/Cargo.toml`!

error: proc-macro derive panicked
 --> src/lib.rs:8:28
  |
8 | #[derive(BorshDeserialize, BorshSerialize)]
  |                            ^^^^^^^^^^^^^^
  |
  = help: message: called `Result::unwrap()` on an `Err` value: Could not find `borsh` in `dependencies` or `dev-dependencies` in `/home/pich/contract/ngo-rs/Cargo.toml`!

error: proc-macro derive panicked
  --> src/lib.rs:14:10
   |
14 | #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
   |          ^^^^^^^^^^^^^^
   |
   = help: message: called `Result::unwrap()` on an `Err` value: Could not find `borsh` in `dependencies` or `dev-dependencies` in `/home/pich/contract/ngo-rs/Cargo.toml`!

error: proc-macro derive panicked
  --> src/lib.rs:14:26
   |
14 | #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
   |                          ^^^^^^^^^^^^^^^^
   |
   = help: message: called `Result::unwrap()` on an `Err` value: Could not find `borsh` in `dependencies` or `dev-dependencies` in `/home/pich/contract/ngo-rs/Cargo.toml`!

error: proc-macro derive panicked
  --> src/lib.rs:28:10
   |
28 | #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
   |          ^^^^^^^^^^^^^^
   |
   = help: message: called `Result::unwrap()` on an `Err` value: Could not find `borsh` in `dependencies` or `dev-dependencies` in `/home/pich/contract/ngo-rs/Cargo.toml`!

error: proc-macro derive panicked
  --> src/lib.rs:28:26
   |
28 | #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
   |                          ^^^^^^^^^^^^^^^^
   |
   = help: message: called `Result::unwrap()` on an `Err` value: Could not find `borsh` in `dependencies` or `dev-dependencies` in `/home/pich/contract/ngo-rs/Cargo.toml`!

error[E0432]: unresolved import `near_sdk::collections`
 --> src/lib.rs:3:15
  |
3 | use near_sdk::collections::{UnorderedMap, Vector};
  |               ^^^^^^^^^^^ could not find `collections` in `near_sdk`

warning: unused import: `self`
 --> src/lib.rs:1:23
  |
1 | use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
  |                       ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused import: `BorshStorageKey`
 --> src/lib.rs:2:46
  |
2 | use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey};
  |                                              ^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0432`.
warning: `ngo-rs` (lib) generated 2 warnings
error: could not compile `ngo-rs` (lib) due to 7 previous errors; 2 warnings emitted