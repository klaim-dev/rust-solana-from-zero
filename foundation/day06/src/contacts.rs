#[derive(Debug, Clone, PartialEq)]
pub struct Contact {
    pub id: u32,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

impl Contact {
    pub fn primary_channel(&self) -> Option<&str> {
        self.email.as_deref().or(self.phone.as_deref())
    }

    pub fn format_contact_line(&self) -> String {
        let email = self.email.as_deref();
        let number = self.phone.as_deref();
        let name = self.name.as_str();

        match email {
            Some(e) => format!("{} <{}>", name, e),
            None => {
                if let Some(num) = number {
                    format!("{} ({})", name, num)
                } else {
                    format!("{} (no contacts)", name)
                }
            }
        }
    }

    pub fn update_email(&mut self, new_email: &str) {
        let trimmed = new_email.trim();

        if trimmed.is_empty() {
            self.email = None;
            return;
        }

        self.email = Some(trimmed.to_string());
    }
}

pub struct Contacts {
    pub items: Vec<Contact>,
}

impl Contacts {
    pub fn find_by_id(&self, id: u32) -> Option<&Contact> {
        self.items.iter().find(|x| x.id == id)
    }

    pub fn find_index_by_name(&self, name: &str) -> Option<usize> {
        self.items
            .iter()
            .enumerate()
            .find(|(_, contact)| contact.name == name)
            .map(|(idx, _)| idx)
    }

    pub fn filter_with_email(&self) -> Vec<&Contact> {
        self.items
            .iter()
            .filter(|contact| contact.email.is_some())
            .collect::<Vec<&Contact>>()
    }

    pub fn first_by_phone_prefix(&self, prefix: &str) -> Option<&Contact> {
        self.items
            .iter()
            .find(|contact| matches!(contact.phone.as_deref(), Some(p) if p.starts_with(prefix)))
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]

    fn find_by_id_basic() {
        let contacts = Contacts {
            items: vec![
                Contact {
                    id: 1,
                    name: "Alice".into(),
                    email: None,
                    phone: None,
                },
                Contact {
                    id: 2,
                    name: "Bob".into(),
                    email: None,
                    phone: None,
                },
            ],
        };

        let c = contacts.find_by_id(1).expect("should exist");

        assert_eq!(c.id, 1);
        assert_eq!(c.name, "Alice");
    }

    #[test]
    fn find_by_id_second() {
        let contacts = Contacts {
            items: vec![
                Contact {
                    id: 1,
                    name: "Alice".into(),
                    email: None,
                    phone: None,
                },
                Contact {
                    id: 2,
                    name: "Bob".into(),
                    email: None,
                    phone: None,
                },
            ],
        };

        let c = contacts.find_by_id(2).expect("should exist");

        assert_eq!(c.id, 2);
        assert_eq!(c.name, "Bob");
    }

    #[test]
    fn find_by_id_not_found() {
        let contacts = Contacts {
            items: vec![
                Contact {
                    id: 1,
                    name: "Alice".into(),
                    email: None,
                    phone: None,
                },
                Contact {
                    id: 2,
                    name: "Bob".into(),
                    email: None,
                    phone: None,
                },
            ],
        };

        let c = contacts.find_by_id(999);

        assert!(c.is_none());
    }

    #[test]

    fn find_index_by_name_basic() {
        let contacts = Contacts {
            items: vec![
                Contact {
                    id: 1,
                    name: "Alice".into(),
                    email: None,
                    phone: None,
                },
                Contact {
                    id: 2,
                    name: "Bob".into(),
                    email: None,
                    phone: None,
                },
            ],
        };

        let idx = contacts
            .find_index_by_name("Alice")
            .expect("Alice should exist");
        assert_eq!(idx, 0);
    }

    #[test]

    fn find_index_by_name_not_exists() {
        let contacts = Contacts {
            items: vec![
                Contact {
                    id: 1,
                    name: "Alice".into(),
                    email: None,
                    phone: None,
                },
                Contact {
                    id: 2,
                    name: "Bob".into(),
                    email: None,
                    phone: None,
                },
            ],
        };

        let idx = contacts.find_index_by_name("Agava");
        assert!(idx.is_none());
    }

    #[test]

    fn primary_channel_basic_email() {
        let contact = Contact {
            id: 1,
            name: "Alice".into(),
            email: Some("alice@gmail.com".into()),
            phone: None,
        };

        let c = contact.primary_channel().expect("Email should exist");
        assert_eq!(c, contact.email.as_deref().unwrap())
    }

    #[test]

    fn primary_channel_basic_phone() {
        let contact = Contact {
            id: 1,
            name: "Alice".into(),
            email: None,
            phone: Some("524-344-256".into()),
        };

        let c = contact.primary_channel().expect("Phone should exist");
        assert_eq!(c, contact.phone.as_deref().unwrap())
    }

    #[test]

    fn primary_channel_email_and_phone_is_none() {
        let contact = Contact {
            id: 1,
            name: "Alice".into(),
            email: None,
            phone: None,
        };

        let c = contact.primary_channel();
        assert!(
            c.is_none(),
            "Expected no primary channel when both email and phone are None"
        )
    }

    #[test]

    fn format_contact_line_some_email() {
        let contact = Contact {
            id: 1,
            name: "Alice".into(),
            email: Some("alice@gmail.com".into()),
            phone: None,
        };

        let string = contact.format_contact_line();
        assert_eq!(
            string,
            format!("{} <{}>", contact.name, contact.email.as_deref().unwrap())
        );
    }

    #[test]

    fn format_contact_line_some_phone() {
        let contact = Contact {
            id: 1,
            name: "Alice".into(),
            email: None,
            phone: Some("256-473-282".into()),
        };

        let string = contact.format_contact_line();
        assert_eq!(
            string,
            format!("{} ({})", contact.name, contact.phone.as_deref().unwrap())
        );
    }

    #[test]
    fn update_email_sets_trimmed_value() {
        let mut contact = Contact {
            id: 1,
            name: "Alice".into(),
            email: None,
            phone: None,
        };

        contact.update_email("   new@example.com  ");

        assert_eq!(contact.email.as_deref(), Some("new@example.com"));
    }

    #[test]
    fn update_email_clears_on_empty_input() {
        let mut contact = Contact {
            id: 1,
            name: "Alice".into(),
            email: Some("old@example.com".into()),
            phone: None,
        };

        contact.update_email("   ");

        assert!(contact.email.is_none());
    }

    #[test]
    fn update_email_empty_string() {
        let mut contact = Contact {
            id: 1,
            name: "Alice".into(),
            email: Some("old@example.com".into()),
            phone: None,
        };

        contact.update_email("");

        assert!(contact.email.is_none());
    }

    #[test]

    fn format_contact_line_phone_and_email_is_none() {
        let contact = Contact {
            id: 1,
            name: "Alice".into(),
            email: None,
            phone: None,
        };

        let string = contact.format_contact_line();
        assert_eq!(string, format!("{} (no contacts)", contact.name))
    }

    #[test]

    fn filter_with_email_basic() {
        let contacts = Contacts {
            items: vec![
                Contact {
                    id: 1,
                    name: "Alice".into(),
                    email: Some("alice@gmail.com".into()),
                    phone: None,
                },
                Contact {
                    id: 2,
                    name: "Bob".into(),
                    email: None,
                    phone: None,
                },
            ],
        };

        let vec_email = contacts.filter_with_email();

        assert_eq!(vec_email.len(), 1);
        assert!(vec_email.iter().any(|x| x.name == "Alice"))
    }

    #[test]

    fn filter_with_email_vec_is_empty() {
        let contacts = Contacts {
            items: vec![
                Contact {
                    id: 1,
                    name: "Alice".into(),
                    email: None,
                    phone: None,
                },
                Contact {
                    id: 2,
                    name: "Bob".into(),
                    email: None,
                    phone: None,
                },
            ],
        };

        let vec_email = contacts.filter_with_email();

        assert!(vec_email.is_empty());
    }

    #[test]
    fn first_by_phone_prefix_basic() {
        let contacts = Contacts {
            items: vec![
                Contact {
                    id: 1,
                    name: "Alice".into(),
                    phone: Some("+4178234".into()),
                    email: None,
                },
                Contact {
                    id: 2,
                    name: "Bob".into(),
                    phone: Some("12345".into()),
                    email: None,
                },
            ],
        };

        let result = contacts
            .first_by_phone_prefix("+41")
            .expect("Should find prefix");
        assert_eq!(result.name, "Alice");
    }

    #[test]
    fn first_by_phone_prefix_miss_prefix() {
        let contacts = Contacts {
            items: vec![
                Contact {
                    id: 1,
                    name: "Alice".into(),
                    phone: Some("+4178234".into()),
                    email: None,
                },
                Contact {
                    id: 2,
                    name: "Bob".into(),
                    phone: Some("12345".into()),
                    email: None,
                },
            ],
        };

        let result = contacts.first_by_phone_prefix("+34");
        assert!(result.is_none());
    }

    #[test]
    fn first_by_phone_prefix_all_empty() {
        let contacts = Contacts {
            items: vec![
                Contact {
                    id: 1,
                    name: "Alice".into(),
                    phone: None,
                    email: None,
                },
                Contact {
                    id: 2,
                    name: "Bob".into(),
                    phone: None,
                    email: None,
                },
            ],
        };

        let result = contacts.first_by_phone_prefix("+41");
        assert!(result.is_none())
    }
}
