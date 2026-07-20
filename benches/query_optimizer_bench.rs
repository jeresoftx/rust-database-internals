use std::hint::black_box;
use std::time::{Duration, Instant};

use rust_database_internals::query_optimizer::{
    ColumnName, CostCatalog, IndexName, IndexStatistics, PhysicalPlan, RelationName,
    RelationStatistics, RowCount, Selectivity,
};

const ROUNDS: usize = 100_000;

fn main() {
    let results = [
        benchmark_table_scan_cost(),
        benchmark_index_scan_cost(),
        benchmark_cost_comparison(),
    ];

    println!("\nQuery Optimizer benchmark educativo");
    println!("Modelo: costo de table scan, index scan y comparación");
    println!("| Operación | Ops | Total | ns/op |");
    println!("|-----------|-----|-------|-------|");

    for result in results {
        println!(
            "| {} | {} | {:?} | {} |",
            result.name,
            result.operations,
            result.elapsed,
            result.nanoseconds_per_operation()
        );
    }
}

struct BenchmarkResult {
    name: &'static str,
    operations: usize,
    elapsed: Duration,
}

impl BenchmarkResult {
    fn nanoseconds_per_operation(&self) -> u128 {
        self.elapsed.as_nanos() / self.operations as u128
    }
}

fn benchmark_table_scan_cost() -> BenchmarkResult {
    let (catalog, table_scan, _) = build_cost_case();
    let start = Instant::now();

    for _ in 0..ROUNDS {
        black_box(
            table_scan
                .estimate_cost(&catalog)
                .expect("table scan estimable"),
        );
    }

    BenchmarkResult {
        name: "estimar table scan",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_index_scan_cost() -> BenchmarkResult {
    let (catalog, _, index_scan) = build_cost_case();
    let start = Instant::now();

    for _ in 0..ROUNDS {
        black_box(
            index_scan
                .estimate_cost(&catalog)
                .expect("index scan estimable"),
        );
    }

    BenchmarkResult {
        name: "estimar index scan",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_cost_comparison() -> BenchmarkResult {
    let (catalog, table_scan, index_scan) = build_cost_case();
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let table_cost = table_scan
            .estimate_cost(&catalog)
            .expect("table scan estimable");
        let index_cost = index_scan
            .estimate_cost(&catalog)
            .expect("index scan estimable");

        black_box(index_cost.is_cheaper_than(&table_cost));
    }

    BenchmarkResult {
        name: "comparar costos",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn build_cost_case() -> (CostCatalog, PhysicalPlan, PhysicalPlan) {
    let relation = RelationName::new("accounts").expect("relación válida");
    let index = IndexName::new("idx_accounts_status").expect("índice válido");
    let catalog = CostCatalog::new(vec![RelationStatistics::new(
        relation.clone(),
        RowCount::new(100_000),
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

    (catalog, table_scan, index_scan)
}
