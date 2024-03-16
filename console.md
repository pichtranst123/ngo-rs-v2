error[E0599]: no function or associated item named `from_yocto` found for struct `NearToken` in the current scope
  --> src/lib.rs:77:43
   |
77 |         let attached_deposit = NearToken::from_yocto(env::attached_deposit());
   |                                           ^^^^^^^^^^
   |                                           |
   |                                           function or associated item not found in `NearToken`
   |                                           help: there is an associated function with a similar name: `from_yoctonear`

error[E0599]: no function or associated item named `from_yocto` found for struct `NearToken` in the current scope
   --> src/lib.rs:125:28
    |
125 |         .map_or(NearToken::from_yocto(0), |donations| donations.iter().map(|donation| donation.amount).sum())
    |                            ^^^^^^^^^^
    |                            |
    |                            function or associated item not found in `NearToken`
    |                            help: there is an associated function with a similar name: `from_yoctonear`

error[E0277]: a value of type `NearToken` cannot be made by summing an iterator over elements of type `NearToken`
    --> src/lib.rs:125:104
     |
125  |         .map_or(NearToken::from_yocto(0), |donations| donations.iter().map(|donation| donation.amount).sum())
     |                                                                                                        ^^^ value of type `NearToken` cannot be made by summing a `std::iter::Iterator<Item=NearToken>`
     |
