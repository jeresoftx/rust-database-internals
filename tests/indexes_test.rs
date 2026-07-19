use rust_database_internals::indexes::{
    ColumnName, IndexDefinition, IndexEntries, IndexEntryKey, IndexError, IndexName, IndexRole,
    IndexTarget, IndexUniqueness, PrimaryKeyValue, SelectivityClass,
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

#[test]
fn empty_index_has_zero_selectivity() {
    let entries = IndexEntries::new(IndexUniqueness::NonUnique);

    let selectivity = entries.selectivity();

    assert_eq!(selectivity.distinct_keys(), 0);
    assert_eq!(selectivity.indexed_rows(), 0);
    assert_eq!(selectivity.ratio(), 0.0);
    assert_eq!(selectivity.class(), SelectivityClass::Empty);
}

#[test]
fn unique_entries_have_high_selectivity() {
    let mut entries = IndexEntries::new(IndexUniqueness::Unique);
    entries
        .insert(
            IndexEntryKey::from("customer-1"),
            PrimaryKeyValue::from("1"),
        )
        .expect("la llave única debe insertarse");
    entries
        .insert(
            IndexEntryKey::from("customer-2"),
            PrimaryKeyValue::from("2"),
        )
        .expect("la llave única debe insertarse");

    let selectivity = entries.selectivity();

    assert_eq!(selectivity.distinct_keys(), 2);
    assert_eq!(selectivity.indexed_rows(), 2);
    assert_eq!(selectivity.ratio(), 1.0);
    assert_eq!(selectivity.class(), SelectivityClass::High);
}

#[test]
fn repeated_values_reduce_selectivity_and_estimate_candidates() {
    let mut entries = IndexEntries::new(IndexUniqueness::NonUnique);
    entries
        .insert(IndexEntryKey::from("MX"), PrimaryKeyValue::from("1"))
        .expect("el índice no único acepta duplicados");
    entries
        .insert(IndexEntryKey::from("MX"), PrimaryKeyValue::from("2"))
        .expect("el índice no único acepta duplicados");
    entries
        .insert(IndexEntryKey::from("US"), PrimaryKeyValue::from("3"))
        .expect("el índice no único acepta otro valor");
    entries
        .insert(IndexEntryKey::from("US"), PrimaryKeyValue::from("4"))
        .expect("el índice no único acepta duplicados");

    let selectivity = entries.selectivity();

    assert_eq!(selectivity.distinct_keys(), 2);
    assert_eq!(selectivity.indexed_rows(), 4);
    assert_eq!(selectivity.ratio(), 0.5);
    assert_eq!(selectivity.class(), SelectivityClass::Medium);
    assert_eq!(
        entries.estimated_candidates_for(&IndexEntryKey::from("MX")),
        2
    );
    assert_eq!(
        entries.estimated_candidates_for(&IndexEntryKey::from("absent")),
        0
    );
}

#[test]
fn heavily_repeated_values_have_low_selectivity() {
    let mut entries = IndexEntries::new(IndexUniqueness::NonUnique);
    for primary_key in ["1", "2", "3", "4"] {
        entries
            .insert(
                IndexEntryKey::from("active"),
                PrimaryKeyValue::from(primary_key),
            )
            .expect("el índice no único acepta duplicados");
    }

    let selectivity = entries.selectivity();

    assert_eq!(selectivity.distinct_keys(), 1);
    assert_eq!(selectivity.indexed_rows(), 4);
    assert_eq!(selectivity.ratio(), 0.25);
    assert_eq!(selectivity.class(), SelectivityClass::Low);
}
