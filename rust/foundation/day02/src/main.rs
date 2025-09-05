mod domain;
mod policy;

use crate::domain::user::User;

fn main() {

let user = User {name: "Alice".to_string(), age: 30};

let result = policy::engine::build_report(&user);
println!("{}", result.summary);



 }

