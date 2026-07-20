use rust_database_internals::query_optimizer::{
    ColumnName, CostCatalog, IndexName, IndexStatistics, PhysicalPlan, RelationName,
    RelationStatistics, RowCount, Selectivity,
};

fn main() {
    let relation = RelationName::new("accounts").expect("la relación debe ser válida");
    let index = IndexName::new("idx_accounts_status").expect("el índice debe ser válido");
    let catalog = CostCatalog::new(vec![RelationStatistics::new(
        relation.clone(),
        RowCount::new(10_000),
    )])
    .with_indexes(vec![IndexStatistics::new(
        index.clone(),
        Selectivity::new_basis_points(500).expect("la selectividad debe ser válida"),
    )]);

    let table_scan = PhysicalPlan::table_scan(relation.clone());
    let index_scan = PhysicalPlan::index_scan(
        relation,
        index,
        ColumnName::new("status").expect("la columna debe ser válida"),
    );

    let table_cost = table_scan
        .estimate_cost(&catalog)
        .expect("table scan debe ser estimable");
    let index_cost = index_scan
        .estimate_cost(&catalog)
        .expect("index scan debe ser estimable");

    assert!(index_cost.is_cheaper_than(&table_cost));

    println!(
        "index scan gana: {} unidades contra {}",
        index_cost.work_units(),
        table_cost.work_units()
    );
}
