pub fn normalize_name(raw: &str) -> String {
    let trimmed = raw.trim().to_lowercase();
    let mut n = trimmed.chars();
    match n.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + n.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_uppercase() {
        assert_eq!(normalize_name("ALICE"), "Alice");
    }

    #[test]
    fn normalize_empty() {
        assert_eq!(normalize_name(""), "");
    }

    #[test]
    fn normalize_lowercase() {
        assert_eq!(normalize_name("alice"), "Alice");
    }
}
