use rust_database_internals::query_optimizer::{
    ColumnName, IndexName, PhysicalAccessPath, PhysicalOperation, PhysicalPlan, RelationName,
};

fn main() {
    let relation = RelationName::new("customers").expect("la relación debe ser válida");
    let index = IndexName::new("idx_customers_email").expect("el índice debe ser válido");
    let lookup_column = ColumnName::new("email").expect("la columna debe ser válida");

    let plan = PhysicalPlan::index_scan(relation.clone(), index.clone(), lookup_column.clone());

    assert_eq!(
        plan.operation(),
        &PhysicalOperation::ReadRelation {
            relation,
            access_path: PhysicalAccessPath::IndexScan {
                index,
                lookup_column,
            },
        }
    );

    println!("solución nivel 2: index scan explícito por email");
}
