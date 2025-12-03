use day13::UserStore;

fn main() {
    // Example usage kept minimal; real logic should live in library and tests.
    let mut store = UserStore::new();
    let _ = store.register(1, "example@example.com", 30);
    if let Some(user) = store.get_by_id(1) {
        println!("Registered user: {:?}", user);
    }
}
