use rust_database_internals::query_optimizer::{
    ColumnName, IndexName, PhysicalAccessPath, PhysicalOperation, PhysicalPlan, RelationName,
};

fn main() {
    let relation = RelationName::new("accounts").expect("la relación debe ser válida");
    let table_scan = PhysicalPlan::table_scan(relation.clone());
    let index_scan = PhysicalPlan::index_scan(
        relation.clone(),
        IndexName::new("idx_accounts_status").expect("el índice debe ser válido"),
        ColumnName::new("status").expect("la columna debe ser válida"),
    );

    assert_eq!(
        table_scan.operation(),
        &PhysicalOperation::ReadRelation {
            relation: relation.clone(),
            access_path: PhysicalAccessPath::TableScan,
        }
    );
    assert!(matches!(
        index_scan.operation(),
        PhysicalOperation::ReadRelation {
            access_path: PhysicalAccessPath::IndexScan { .. },
            ..
        }
    ));

    println!("dos rutas físicas: TableScan e IndexScan");
}
