use rust_database_internals::indexes::{
    ColumnName, IndexDefinition, IndexEntries, IndexEntryKey, IndexError, IndexName, IndexRole,
    IndexTarget, IndexUniqueness, PrimaryKeyValue,
};

#[test]
fn primary_index_declares_record_pointer_target() {
    let index = IndexDefinition::primary(
        IndexName::new("pk_customers").expect("el nombre del índice debe ser válido"),
        ColumnName::new("customer_id").expect("la columna debe ser válida"),
    );

    assert_eq!(index.name().as_str(), "pk_customers");
    assert_eq!(index.role(), IndexRole::Primary);
    assert_eq!(index.uniqueness(), IndexUniqueness::Unique);
    assert_eq!(
        index.key_columns(),
        &[ColumnName::new("customer_id").unwrap()]
    );
    assert_eq!(index.target(), &IndexTarget::RecordPointer);
}

#[test]
fn secondary_index_declares_primary_key_target() {
    let index = IndexDefinition::secondary(
        IndexName::new("idx_customers_email").expect("el nombre del índice debe ser válido"),
        ColumnName::new("email").expect("la columna secundaria debe ser válida"),
        ColumnName::new("customer_id").expect("la primary key debe ser válida"),
    );

    assert_eq!(index.name().as_str(), "idx_customers_email");
    assert_eq!(index.role(), IndexRole::Secondary);
    assert_eq!(index.uniqueness(), IndexUniqueness::NonUnique);
    assert_eq!(index.key_columns(), &[ColumnName::new("email").unwrap()]);
    assert_eq!(
        index.target(),
        &IndexTarget::PrimaryKey(ColumnName::new("customer_id").unwrap())
    );
}

#[test]
fn unique_secondary_index_can_be_declared() {
    let index = IndexDefinition::unique_secondary(
        IndexName::new("uq_customers_email").expect("el nombre del índice debe ser válido"),
        ColumnName::new("email").expect("la columna secundaria debe ser válida"),
        ColumnName::new("customer_id").expect("la primary key debe ser válida"),
    );

    assert_eq!(index.name().as_str(), "uq_customers_email");
    assert_eq!(index.role(), IndexRole::Secondary);
    assert_eq!(index.uniqueness(), IndexUniqueness::Unique);
    assert_eq!(index.key_columns(), &[ColumnName::new("email").unwrap()]);
    assert_eq!(
        index.target(),
        &IndexTarget::PrimaryKey(ColumnName::new("customer_id").unwrap())
    );
}

#[test]
fn index_name_rejects_blank_text() {
    assert_eq!(IndexName::new(""), Err(IndexError::BlankIndexName));
    assert_eq!(IndexName::new("   "), Err(IndexError::BlankIndexName));
}

#[test]
fn column_name_rejects_blank_text() {
    assert_eq!(ColumnName::new(""), Err(IndexError::BlankColumnName));
    assert_eq!(ColumnName::new("   "), Err(IndexError::BlankColumnName));
}

#[test]
fn unique_index_rejects_duplicate_entry_key() {
    let mut entries = IndexEntries::new(IndexUniqueness::Unique);

    assert_eq!(
        entries.insert(
            IndexEntryKey::from("ana@example.com"),
            PrimaryKeyValue::from("customer-1")
        ),
        Ok(())
    );

    assert_eq!(
        entries.insert(
            IndexEntryKey::from("ana@example.com"),
            PrimaryKeyValue::from("customer-2")
        ),
        Err(IndexError::DuplicateIndexKey(IndexEntryKey::from(
            "ana@example.com"
        )))
    );
    assert_eq!(
        entries.primary_keys_for(&IndexEntryKey::from("ana@example.com")),
        vec![PrimaryKeyValue::from("customer-1")]
    );
}

#[test]
fn non_unique_index_allows_duplicate_entry_key() {
    let mut entries = IndexEntries::new(IndexUniqueness::NonUnique);

    assert_eq!(
        entries.insert(
            IndexEntryKey::from("mx"),
            PrimaryKeyValue::from("customer-1")
        ),
        Ok(())
    );
    assert_eq!(
        entries.insert(
            IndexEntryKey::from("mx"),
            PrimaryKeyValue::from("customer-2")
        ),
        Ok(())
    );

    assert_eq!(
        entries.primary_keys_for(&IndexEntryKey::from("mx")),
        vec![
            PrimaryKeyValue::from("customer-1"),
            PrimaryKeyValue::from("customer-2")
        ]
    );
}

#[test]
fn index_entries_return_empty_result_for_absent_key() {
    let entries = IndexEntries::new(IndexUniqueness::NonUnique);

    assert_eq!(
        entries.primary_keys_for(&IndexEntryKey::from("absent")),
        Vec::<PrimaryKeyValue>::new()
    );
}
