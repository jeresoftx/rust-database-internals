use rust_database_internals::query_optimizer::{
    ColumnName, CostCatalog, IndexName, IndexStatistics, PhysicalPlan, RelationName,
    RelationStatistics, RowCount, Selectivity,
};

fn main() {
    let relation = RelationName::new("customers").expect("la relación debe ser válida");
    let index = IndexName::new("idx_customers_email").expect("el índice debe ser válido");
    let catalog = CostCatalog::new(vec![RelationStatistics::new(
        relation.clone(),
        RowCount::new(50_000),
    )])
    .with_indexes(vec![IndexStatistics::new(
        index.clone(),
        Selectivity::new_basis_points(20).expect("la selectividad debe ser válida"),
    )]);

    let table_scan = PhysicalPlan::table_scan(relation.clone());
    let index_scan = PhysicalPlan::index_scan(
        relation,
        index,
        ColumnName::new("email").expect("la columna debe ser válida"),
    );

    let table_cost = table_scan
        .estimate_cost(&catalog)
        .expect("table scan debe ser estimable");
    let index_cost = index_scan
        .estimate_cost(&catalog)
        .expect("index scan debe ser estimable");

    assert_eq!(table_cost.work_units(), 50_000);
    assert_eq!(index_cost.work_units(), 110);
    assert!(index_cost.is_cheaper_than(&table_cost));

    println!("solución nivel 3: el index scan tiene menor costo estimado");
}
