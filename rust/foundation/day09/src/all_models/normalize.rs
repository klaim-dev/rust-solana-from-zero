   use crate::errors::user_errors::UserError;
   

   
    pub fn name_ascii(name: &str) -> Result<String, UserError> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(UserError::EmptyName);
        };

        let mut name_chars = trimmed.chars();
        let first = name_chars.next().unwrap().to_ascii_uppercase();
        let rest = name_chars.as_str().to_ascii_lowercase();
        Ok(format!("{}{}", first, rest))


    }