
pub struct User {
    pub name: String,
    pub age: u32,
}
impl User {
    pub fn greet(name: &str, age: u32) -> Result<(String, User), String>  {
        let user = User{
            name: name.to_string(),
            age,
        };

        if user.name.is_empty() {
           return Err("Input name is empty".to_string());
        };

        let trim = user.name.trim();
        let first_slice = user.name[..1].to_ascii_uppercase();
        let slice = user.name[1..].to_ascii_lowercase();



        Ok((format!("Hello {} ({})", first_slice + &slice, user.age), user))
} 

pub fn name_len(user: &User) -> usize {
    user.name.len()
}
pub fn name_cap(user: &User) -> usize {
    user.name.capacity()
}

pub fn report_string_layout(user: &User) -> (usize, usize) {
    let len = User::name_len(user);
    let capacity = User::name_cap(user);
    println!("User name: {}, len: {}, capacity: {}", user.name, len, capacity);
    (len, capacity)
    
}
}