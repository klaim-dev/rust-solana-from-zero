use day10::registry::UserRegistry;

fn main() {
    let mut user_registration = UserRegistry::new();
    let user = user_registration.register(1, "alice@gmail.com".to_string(), 32);
    match user {
        Ok(u) => println!("User registered: {}", u.id()),
        Err(e) => println!("Error: {}", e),
    };
    println!("Registry: {:?}", user_registration.all())
}
