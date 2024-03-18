   Compiling ngo-rs v0.1.0 (/home/pich/contract/ngo-rs)
error[E0382]: use of moved value: `project_name`
  --> src/lib.rs:73:30
   |
51 |     pub fn create_project(&mut self, project_name: String, project_description: String, target_amount: u128, ipfs_image: String, ipfs_hash: Vec<String>, duration: u8) {
   |                                      ------------ move occurs because `project_name` has type `std::string::String`, which does not implement the `Copy` trait
...
64 |             project_name,
   |             ------------ value moved here
...
73 |         self.projects.insert(project_name, project_metadata);
   |                              ^^^^^^^^^^^^ value used here after move
   |
help: consider cloning the value if the performance cost is acceptable
   |
64 |             project_name: project_name.clone(),
   |                         ++++++++++++++++++++++

error[E0507]: cannot move out of `project.creator_id` which is behind a shared reference
  --> src/lib.rs:94:22
   |
94 |         Promise::new(project.creator_id).transfer(attached_deposit);
   |                      ^^^^^^^^^^^^^^^^^^ move occurs because `project.creator_id` has type `AccountId`, which does not implement the `Copy` trait

Some errors have detailed explanations: E0382, E0507.
For more information about an error, try `rustc --explain E0382`.
error: could not compile `ngo-rs` (lib) due to 2 previous errors
