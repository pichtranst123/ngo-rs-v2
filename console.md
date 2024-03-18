error[E0599]: no function or associated item named `from_str` found for struct `NearToken` in the current scope
  --> src/lib.rs:83:42
   |
83 |         let donation_amount = NearToken::from_str(&amount).expect("Invalid amount format");
   |                                          ^^^^^^^^ function or associated item not found in `NearToken`
