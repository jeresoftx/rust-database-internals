use rust_database_internals::acid::{AcidError, UniqueConstraint};

fn main() {
    let mut emails =
        UniqueConstraint::new("customers.email").expect("la invariante debe tener nombre");

    emails
        .insert("ana@example.com")
        .expect("el primer correo debe aceptarse");

    assert_eq!(
        emails.insert("ana@example.com"),
        Err(AcidError::ConsistencyViolation {
            invariant: "customers.email".to_owned(),
            value: "ana@example.com".to_owned(),
        })
    );

    assert_eq!(emails.len(), 1);

    println!("Consistency: la constraint evitó un correo duplicado.");
}
