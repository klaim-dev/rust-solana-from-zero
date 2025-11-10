mod domain;
mod utils;

use domain::user::{greet, name_len, new_user, user_is_adult};

fn main() {
    let user = new_user("ALICE", 32);
    println!("{}", greet(&user));
    println!("len = {}", name_len(&user));
    println!("adult = {}", user_is_adult(&user));
}
