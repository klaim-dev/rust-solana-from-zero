use crate::utils::normalize::normalize_name;

pub struct User {
    name: String,
    age: u32,
}

pub fn greet(user: &User) -> String {
    format!("Hello, {} ({})!", user.name, user.age)
}

pub fn name_len(user: &User) -> usize {
    user.name.len()
}
pub fn user_is_adult(user: &User) -> bool {
    user.age >= 18
}
pub fn report_name_layout(user: &User) -> (usize, usize) {
    (user.name.len(), user.name.capacity())
}

pub fn new_user(raw_name: &str, age: u32) -> User {
    let name = normalize_name(raw_name);
    User { name, age }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_basic() {
        let alice = User {
            name: "Alice".to_string(),
            age: 32,
        };
        let bob = User {
            name: "Bob".to_string(),
            age: 17,
        };

        assert_eq!(greet(&alice), "Hello, Alice (32)!");
        assert_eq!(name_len(&alice), 5);

        assert!(user_is_adult(&alice));
        assert!(!user_is_adult(&bob));

        let (len, cap) = report_name_layout(&alice);
        assert_eq!(len, 5);
        assert!(cap >= len);
    }
}
