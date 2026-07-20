use rust_database_internals::query_optimizer::{
    ColumnName, ComparisonOperator, Literal, LogicalOperation, LogicalPlan, Predicate, RelationName,
};

fn main() {
    let plan =
        LogicalPlan::relation(RelationName::new("accounts").expect("la relación debe ser válida"))
            .select(Predicate::comparison(
                ColumnName::new("status").expect("la columna debe ser válida"),
                ComparisonOperator::Eq,
                Literal::text("active"),
            ))
            .project(vec![
                ColumnName::new("id").expect("la columna debe ser válida"),
                ColumnName::new("balance").expect("la columna debe ser válida"),
            ])
            .expect("la proyección debe pedir columnas");

    assert!(matches!(plan.operation(), LogicalOperation::Project { .. }));
    assert_eq!(plan.children().len(), 1);

    println!("plan lógico: Project -> Select -> ReadRelation");
}
