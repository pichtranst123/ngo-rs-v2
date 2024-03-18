error[E0308]: mismatched types
  --> src/lib.rs:82:32
   |
82 |     assert!(attached_deposit > 0, "Donation must be greater than 0");
   |             ----------------   ^ expected `NearToken`, found integer
   |             |
   |             expected because this is `NearToken`

error[E0308]: mismatched types
  --> src/lib.rs:92:17
   |
92 |         amount: attached_deposit, // Use attached_deposit directly as it's already a u128
   |                 ^^^^^^^^^^^^^^^^ expected `u128`, found `NearToken`

error[E0277]: cannot add-assign `NearToken` to `u128`
  --> src/lib.rs:88:30
   |
88 |     project.collected_amount += attached_deposit;
   |                              ^^ no implementation for `u128 += NearToken`
   |
   = help: the trait `AddAssign<NearToken>` is not implemented for `u128`
   = help: the following other types implement trait `AddAssign<Rhs>`:
             <u128 as AddAssign>
             <u128 as AddAssign<&u128>>

error[E0308]: mismatched types
   --> src/lib.rs:111:55
    |
111 |     Promise::new(project.creator_id.clone()).transfer(project.collected_amount)
    |                                              -------- ^^^^^^^^^^^^^^^^^^^^^^^^ expected `NearToken`, found `u128`
    |                                              |
    |                                              arguments to this method are incorrect
    |
note: method defined here
   --> /home/pich/.cargo/registry/src/index.crates.io-6f17d22bba15001f/near-sdk-5.0.0/src/promise.rs:317:12
