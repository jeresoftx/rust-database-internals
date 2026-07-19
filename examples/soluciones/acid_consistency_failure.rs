use rust_database_internals::acid::{AcidError, UniqueConstraint};

fn main() {
    let mut email_constraint =
        UniqueConstraint::new("customers.email").expect("la invariante debe tener nombre");

    email_constraint
        .insert("ana@example.com")
        .expect("el primer correo debe aceptarse");

    let duplicate = email_constraint.insert("ana@example.com");

    assert_eq!(
        duplicate,
        Err(AcidError::ConsistencyViolation {
            invariant: "customers.email".to_owned(),
            value: "ana@example.com".to_owned(),
        })
    );
    assert_eq!(email_constraint.len(), 1);
    assert!(email_constraint.contains("ana@example.com"));

    println!("Ejercicio completado: la inconsistencia fue rechazada.");
}
