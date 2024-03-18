error[E0271]: expected `new` to be a fn item that returns `&Vec<Donation>`, but it returns `Vec<_>`
   --> src/lib.rs:91:76
    |
91  |         let mut donations = self.donations.get(&project_id).unwrap_or_else(Vec::new);
    |                                                             -------------- ^^^^^^^^ expected `&Vec<Donation>`, found `Vec<_>`
    |                                                             |
    |                                                             required by a bound introduced by this call
    |
    = note: expected reference `&Vec<Donation>`
                  found struct `Vec<_>`
note: required by a bound in `std::option::Option::<T>::unwrap_or_else`
   --> /home/pich/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:975:24
    |
973 |     pub fn unwrap_or_else<F>(self, f: F) -> T
    |            -------------- required by a bound in this associated function
974 |     where
975 |         F: FnOnce() -> T,
    |                        ^ required by this bound in `Option::<T>::unwrap_or_else`

error[E0308]: arguments to this method are incorrect
    --> src/lib.rs:93:24
     |
93   |         self.donations.insert(&project_id, &donations);
     |                        ^^^^^^ ----------- expected `String`, found `&String`
     |
note: expected `Vec<Donation>`, found `&&Vec<Donation>`
    --> src/lib.rs:93:44
     |
93   |         self.donations.insert(&project_id, &donations);
     |                                            ^^^^^^^^^^
     = note: expected struct `Vec<Donation>`
             found reference `&&Vec<Donation>`
help: the return type of this call is `&&Vec<Donation>` due to the type of the argument passed
    --> src/lib.rs:93:9
     |
93   |         self.donations.insert(&project_id, &donations);
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^----------^
     |                                            |
     |                                            this argument influences the return type of `insert`
note: method defined here
    --> /home/pich/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/collections/hash/map.rs:1104:12
     |
1104 |     pub fn insert(&mut self, k: K, v: V) -> Option<V> {
     |            ^^^^^^
help: consider removing the borrow
     |
93   -         self.donations.insert(&project_id, &donations);
93   +         self.donations.insert(project_id, &donations);
     |
help: try using a conversion method
     |
93   |         self.donations.insert(&project_id, (&donations).to_vec());
     |                                            +          ++++++++++

error[E0308]: mismatched types
  --> src/lib.rs:96:9
   |
76 |     pub fn create_donate(&mut self, project_id: String) {
   |                                                         - expected `()` because of default return type
...
96 |         Promise::new(project.creator_id).transfer(attached_deposit)
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- help: consider using a semicolon here: `;`
   |         |
   |         expected `()`, found `Promise`

Some errors have detailed explanations: E0271, E0308.
For more information about an error, try `rustc --explain E0271`.
error: could not compile `ngo-rs` (lib) due to 3 previous errors
