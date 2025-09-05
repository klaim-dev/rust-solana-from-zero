use crate::domain::user::User;

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
