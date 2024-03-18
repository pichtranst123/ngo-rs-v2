error[E0308]: mismatched types
  --> src/lib.rs:81:36
   |
81 |         assert!(attached_deposit > 0, "Donation must be greater than 0");
   |                 ----------------   ^ expected `NearToken`, found integer
   |                 |
   |                 expected because this is `NearToken`

error[E0308]: mismatched types
  --> src/lib.rs:90:21
   |
90 |             amount: attached_deposit,
   |                     ^^^^^^^^^^^^^^^^ expected `u128`, found `NearToken`

error[E0277]: cannot add-assign `NearToken` to `u128`
  --> src/lib.rs:86:34
   |
86 |         project.collected_amount += attached_deposit;
   |                                  ^^ no implementation for `u128 += NearToken`
   |
   = help: the trait `AddAssign<NearToken>` is not implemented for `u128`
   = help: the following other types implement trait `AddAssign<Rhs>`:
             <u128 as AddAssign>
             <u128 as AddAssign<&u128>>

error[E0308]: mismatched types
   --> src/lib.rs:107:59
    |
107 |         Promise::new(project.creator_id.clone()).transfer(project.collected_amount)
    |                                                  -------- ^^^^^^^^^^^^^^^^^^^^^^^^ expected `NearToken`, found `u128`
    |                                                  |
    |                                                  arguments to this method are incorrect
    |
