use crate::domain::user::User;

    mod domain;
    mod utiles;

fn main() {
    let name = String::from("Alice");
    let age = 30;
    let user;
    let greet= utiles::normalize::greet(&name, age);
    match greet {
        Ok((v, u)) => {
            println!("{}", v);
            user = u;
        }
        Err(e) => {
            println!("{}", e);
        return;
    }
    };

    let len_name = User::name_len(&user);
    println!("{}", len_name);

    let capacity = User::name_cap(&user);
    println!("{}", capacity);

    let (l,c) = User::report_string_layout(&user);

    
}