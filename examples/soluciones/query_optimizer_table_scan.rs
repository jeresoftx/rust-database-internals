use rust_database_internals::query_optimizer::{
    PhysicalAccessPath, PhysicalOperation, PhysicalPlan, RelationName,
};

fn main() {
    let relation = RelationName::new("customers").expect("la relación debe ser válida");
    let plan = PhysicalPlan::table_scan(relation.clone());

    assert_eq!(
        plan.operation(),
        &PhysicalOperation::ReadRelation {
            relation,
            access_path: PhysicalAccessPath::TableScan,
        }
    );

    println!("solución nivel 1: table scan explícito");
}
