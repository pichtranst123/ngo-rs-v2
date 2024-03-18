error[E0308]: mismatched types
  --> src/lib.rs:80:36
   |
80 |         assert!(attached_deposit > 0, "Donation must be greater than 0");
   |                 ----------------   ^ expected `NearToken`, found integer
   |                 |
   |                 expected because this is `NearToken`

error[E0610]: `u128` is a primitive type and therefore doesn't have fields
  --> src/lib.rs:85:58
   |
85 |         let new_collected_amount = project.target_amount.0 + attached_deposit;
   |                                                          ^

error[E0308]: arguments to this method are incorrect
    --> src/lib.rs:88:23
     |
88   |         self.projects.insert(&project_id, &project);
     |                       ^^^^^^ -----------  -------- expected `Project`, found `&&Project`
     |                              |
     |                              expected `String`, found `&String`
     |
help: the return type of this call is `&&Project` due to the type of the argument passed
    --> src/lib.rs:88:9
     |
88   |         self.projects.insert(&project_id, &project);
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^--------^
     |                                           |
     |                                           this argument influences the return type of `insert`
note: method defined here
    --> /home/pich/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/collections/hash/map.rs:1104:12
     |
1104 |     pub fn insert(&mut self, k: K, v: V) -> Option<V> {
     |            ^^^^^^
help: consider removing the borrow
     |
88   -         self.projects.insert(&project_id, &project);
88   +         self.projects.insert(project_id, &project);
     |

error[E0308]: mismatched types
  --> src/lib.rs:92:21
   |
92 |             amount: attached_deposit,
   |                     ^^^^^^^^^^^^^^^^ expected `u128`, found `NearToken`

error[E0610]: `u128` is a primitive type and therefore doesn't have fields
   --> src/lib.rs:105:39
    |
105 |         assert!(project.target_amount.0 >= project.target_amount.0, "Target amount not reached");
    |                                       ^

error[E0610]: `u128` is a primitive type and therefore doesn't have fields
   --> src/lib.rs:105:66
    |
105 |         assert!(project.target_amount.0 >= project.target_amount.0, "Target amount not reached");
    |                                                                  ^

error[E0308]: arguments to this method are incorrect
    --> src/lib.rs:108:23
     |
108  |         self.projects.insert(&project_id, &project);
     |                       ^^^^^^ -----------  -------- expected `Project`, found `&&Project`
     |                              |
     |                              expected `String`, found `&String`
     |
help: the return type of this call is `&&Project` due to the type of the argument passed
    --> src/lib.rs:108:9
     |
108  |         self.projects.insert(&project_id, &project);
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^--------^
     |                                           |
     |                                           this argument influences the return type of `insert`
note: method defined here
    --> /home/pich/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/collections/hash/map.rs:1104:12
     |
1104 |     pub fn insert(&mut self, k: K, v: V) -> Option<V> {
     |            ^^^^^^
help: consider removing the borrow
     |
108  -         self.projects.insert(&project_id, &project);
108  +         self.projects.insert(project_id, &project);
     |

error[E0610]: `u128` is a primitive type and therefore doesn't have fields
   --> src/lib.rs:110:81
    |
110 |         Promise::new(project.creator_id.clone()).transfer(project.target_amount.0)
    |                                                                                 ^

Some errors have detailed explanations: E0308, E0610.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `ngo-rs` (lib) due to 8 previous errors
