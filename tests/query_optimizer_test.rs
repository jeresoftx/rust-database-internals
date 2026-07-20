use rust_database_internals::query_optimizer::{
    ColumnName, ComparisonOperator, Literal, LogicalOperation, LogicalPlan, PhysicalAccessPath,
    PhysicalOperation, PhysicalPlan, Predicate, QueryOptimizerError, RelationName,
};

#[test]
fn logical_plan_represents_query_intent_without_execution_choices() {
    let plan = LogicalPlan::relation(RelationName::new("accounts").expect("relación válida"))
        .select(Predicate::comparison(
            ColumnName::new("status").expect("columna válida"),
            ComparisonOperator::Eq,
            Literal::text("active"),
        ))
        .project(vec![
            ColumnName::new("id").expect("columna válida"),
            ColumnName::new("balance").expect("columna válida"),
        ])
        .expect("proyección válida");

    assert_eq!(
        plan.operation(),
        &LogicalOperation::Project {
            columns: vec![
                ColumnName::new("id").expect("columna válida"),
                ColumnName::new("balance").expect("columna válida"),
            ],
        }
    );
    assert_eq!(plan.children().len(), 1);
    assert!(matches!(
        plan.children()[0].operation(),
        LogicalOperation::Select { .. }
    ));
    assert!(matches!(
        plan.children()[0].children()[0].operation(),
        LogicalOperation::ReadRelation { .. }
    ));
}

#[test]
fn physical_plan_represents_execution_shape_separately_from_logical_plan() {
    let relation = RelationName::new("accounts").expect("relación válida");
    let predicate = Predicate::comparison(
        ColumnName::new("status").expect("columna válida"),
        ComparisonOperator::Eq,
        Literal::text("active"),
    );

    let plan = PhysicalPlan::read_relation(relation.clone(), PhysicalAccessPath::Unchosen)
        .filter(predicate.clone())
        .project(vec![ColumnName::new("id").expect("columna válida")])
        .expect("proyección válida");

    assert_eq!(
        plan.operation(),
        &PhysicalOperation::Project {
            columns: vec![ColumnName::new("id").expect("columna válida")],
        }
    );
    assert_eq!(plan.children().len(), 1);
    assert_eq!(
        plan.children()[0].operation(),
        &PhysicalOperation::Filter { predicate }
    );
    assert_eq!(
        plan.children()[0].children()[0].operation(),
        &PhysicalOperation::ReadRelation {
            relation,
            access_path: PhysicalAccessPath::Unchosen,
        }
    );
}

#[test]
fn representation_rejects_blank_names_and_empty_projection() {
    assert_eq!(
        RelationName::new("   "),
        Err(QueryOptimizerError::BlankRelationName)
    );
    assert_eq!(
        ColumnName::new("   "),
        Err(QueryOptimizerError::BlankColumnName)
    );

    let plan = LogicalPlan::relation(RelationName::new("accounts").expect("relación válida"));

    assert_eq!(
        plan.project(vec![]),
        Err(QueryOptimizerError::ProjectionRequiresColumns)
    );
}
