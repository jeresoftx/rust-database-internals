use rust_database_internals::indexes::{
    ColumnName, IndexDefinition, IndexError, IndexName, IndexRole, IndexTarget,
};

#[test]
fn primary_index_declares_record_pointer_target() {
    let index = IndexDefinition::primary(
        IndexName::new("pk_customers").expect("el nombre del índice debe ser válido"),
        ColumnName::new("customer_id").expect("la columna debe ser válida"),
    );

    assert_eq!(index.name().as_str(), "pk_customers");
    assert_eq!(index.role(), IndexRole::Primary);
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
