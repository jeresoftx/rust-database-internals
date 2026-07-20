use rust_database_internals::query_optimizer::{
    ColumnName, ComparisonOperator, CostCatalog, IndexName, IndexStatistics, Literal,
    LogicalOperation, LogicalPlan, PhysicalAccessPath, PhysicalOperation, PhysicalPlan, Predicate,
    QueryOptimizerError, RelationName, RelationStatistics, RowCount, Selectivity,
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
fn table_scan_represents_full_relation_access() {
    let relation = RelationName::new("accounts").expect("relación válida");

    let plan = PhysicalPlan::table_scan(relation.clone());

    assert_eq!(
        plan.operation(),
        &PhysicalOperation::ReadRelation {
            relation,
            access_path: PhysicalAccessPath::TableScan,
        }
    );
    assert!(plan.children().is_empty());
}

#[test]
fn index_scan_represents_access_through_named_index() {
    let relation = RelationName::new("accounts").expect("relación válida");
    let index = IndexName::new("idx_accounts_status").expect("índice válido");
    let lookup_column = ColumnName::new("status").expect("columna válida");

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
    assert!(plan.children().is_empty());
}

#[test]
fn table_scan_cost_reads_every_row_in_relation() {
    let relation = RelationName::new("accounts").expect("relación válida");
    let catalog = CostCatalog::new(vec![RelationStatistics::new(
        relation.clone(),
        RowCount::new(10_000),
    )]);
    let plan = PhysicalPlan::table_scan(relation);

    let cost = plan.estimate_cost(&catalog).expect("costo estimable");

    assert_eq!(cost.rows_read(), 10_000);
    assert_eq!(cost.rows_output(), 10_000);
    assert_eq!(cost.work_units(), 10_000);
}

#[test]
fn index_scan_cost_uses_index_selectivity() {
    let relation = RelationName::new("accounts").expect("relación válida");
    let index = IndexName::new("idx_accounts_status").expect("índice válido");
    let catalog = CostCatalog::new(vec![RelationStatistics::new(
        relation.clone(),
        RowCount::new(10_000),
    )])
    .with_indexes(vec![IndexStatistics::new(
        index.clone(),
        Selectivity::new_basis_points(500).expect("selectividad válida"),
    )]);
    let plan = PhysicalPlan::index_scan(
        relation,
        index,
        ColumnName::new("status").expect("columna válida"),
    );

    let cost = plan.estimate_cost(&catalog).expect("costo estimable");

    assert_eq!(cost.rows_read(), 500);
    assert_eq!(cost.rows_output(), 500);
    assert_eq!(cost.work_units(), 510);
}

#[test]
fn cheaper_cost_can_be_compared_between_physical_plans() {
    let relation = RelationName::new("accounts").expect("relación válida");
    let index = IndexName::new("idx_accounts_status").expect("índice válido");
    let catalog = CostCatalog::new(vec![RelationStatistics::new(
        relation.clone(),
        RowCount::new(10_000),
    )])
    .with_indexes(vec![IndexStatistics::new(
        index.clone(),
        Selectivity::new_basis_points(500).expect("selectividad válida"),
    )]);
    let table_scan = PhysicalPlan::table_scan(relation.clone());
    let index_scan = PhysicalPlan::index_scan(
        relation,
        index,
        ColumnName::new("status").expect("columna válida"),
    );

    let table_cost = table_scan
        .estimate_cost(&catalog)
        .expect("table scan estimable");
    let index_cost = index_scan
        .estimate_cost(&catalog)
        .expect("index scan estimable");

    assert!(index_cost.is_cheaper_than(&table_cost));
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
    assert_eq!(
        IndexName::new("   "),
        Err(QueryOptimizerError::BlankIndexName)
    );

    let plan = LogicalPlan::relation(RelationName::new("accounts").expect("relación válida"));

    assert_eq!(
        plan.project(vec![]),
        Err(QueryOptimizerError::ProjectionRequiresColumns)
    );
    assert_eq!(
        Selectivity::new_basis_points(10_001),
        Err(QueryOptimizerError::InvalidSelectivity)
    );
}
