pich@pich-VirtualBox:~/contract/ngo-rs$ cargo build --target wasm32-unknown-unknown --release
   Compiling ngo-rs v0.1.0 (/home/pich/contract/ngo-rs)
error[E0308]: mismatched types
  --> src/lib.rs:76:58
   |
76 |         let attached_deposit = NearToken::from_yoctonear(env::attached_deposit());
   |                                ------------------------- ^^^^^^^^^^^^^^^^^^^^^^^ expected `u128`, found `NearToken`
   |                                |
   |                                arguments to this function are incorrect
   |
note: associated function defined here
  --> /home/pich/.cargo/registry/src/index.crates.io-6f17d22bba15001f/near-token-0.2.0/src/lib.rs:58:18
   |
58 |     pub const fn from_yoctonear(inner: u128) -> Self {
   |                  ^^^^^^^^^^^^^^

error[E0369]: cannot add `NearToken` to `NearToken`
   --> src/lib.rs:110:41
    |
110 |             .reduce(|acc, donation| acc + donation)
    |                                     --- ^ -------- NearToken
    |                                     |
    |                                     NearToken

Some errors have detailed explanations: E0308, E0369.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `ngo-rs` (lib) due to 2 previous errors
