error[E0308]: mismatched types
  --> src/lib.rs:66:29
   |
66 |             current_amount: 0,
   |                             ^ expected `NearToken`, found integer

error[E0369]: cannot add `NearToken` to `NearToken`
  --> src/lib.rs:84:59
   |
84 |         let potential_new_amount = project.current_amount + donation_amount;
   |                                    ---------------------- ^ --------------- NearToken
   |                                    |
   |                                    NearToken

error[E0308]: mismatched types
   --> src/lib.rs:103:34
    |
103 |         project.current_amount = 0;
    |         ----------------------   ^ expected `NearToken`, found integer
    |         |
    |         expected due to the type of this binding

Some errors have detailed explanations: E0308, E0369, E0432.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `ngo-rs` (lib) due to 4 previous errors
