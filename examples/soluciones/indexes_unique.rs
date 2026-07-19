use rust_database_internals::indexes::{
    IndexEntries, IndexEntryKey, IndexError, IndexUniqueness, PrimaryKeyValue,
};

fn main() {
    let mut email_index = IndexEntries::new(IndexUniqueness::Unique);

    email_index
        .insert(
            IndexEntryKey::from("ana@example.com"),
            PrimaryKeyValue::from("1001"),
        )
        .expect("el primer correo debe insertarse");

    assert_eq!(
        email_index.insert(
            IndexEntryKey::from("ana@example.com"),
            PrimaryKeyValue::from("1002"),
        ),
        Err(IndexError::DuplicateIndexKey(IndexEntryKey::from(
            "ana@example.com"
        )))
    );

    println!("El índice único rechazó un correo duplicado.");
}
