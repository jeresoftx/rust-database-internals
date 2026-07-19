use rust_database_internals::indexes::{ColumnName, IndexDefinition, IndexName};

fn main() {
    let primary = IndexDefinition::primary(
        IndexName::new("pk_customers").expect("el nombre del índice debe ser válido"),
        ColumnName::new("customer_id").expect("la columna debe ser válida"),
    );

    assert_eq!(primary.name().as_str(), "pk_customers");
    assert_eq!(primary.key_columns()[0].as_str(), "customer_id");

    println!(
        "{}({}) es el camino canónico hacia el registro",
        primary.name().as_str(),
        primary.key_columns()[0].as_str()
    );
}
