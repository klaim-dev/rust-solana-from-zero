mod contacts;

use contacts::{Contact, Contacts};

fn main() {
    let contacts = Contacts {
        items: vec![
            Contact {
                id: 1,
                name: "Alice".into(),
                email: Some("alice@example.com".into()),
                phone: Some("+44123456789".into()),
            },
            Contact {
                id: 2,
                name: "Bob".into(),
                email: None,
                phone: Some("+14155552671".into()),
            },
            Contact {
                id: 3,
                name: "Charlie".into(),
                email: Some("charlie@example.org".into()),
                phone: None,
            },
        ],
    };

    println!("Contact directory demo");
    println!("----------------------");

    for contact in &contacts.items {
        println!("{}", contact.format_contact_line());
    }

    if let Some(idx) = contacts.find_index_by_name("Alice") {
        println!("\nAlice is stored at index {}", idx);
    }

    let email_contacts = contacts.filter_with_email();
    if !email_contacts.is_empty() {
        println!(
            "\n{} contact(s) have an email address:",
            email_contacts.len()
        );
        for contact in email_contacts {
            println!(" - {}", contact.format_contact_line());
        }
    }

    match contacts.first_by_phone_prefix("+44") {
        Some(contact) => println!(
            "\nFirst contact with UK prefix: {}",
            contact.format_contact_line()
        ),
        None => println!("\nNo contacts with a UK phone prefix."),
    }
}
