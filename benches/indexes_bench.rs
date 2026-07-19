use std::hint::black_box;
use std::time::{Duration, Instant};

use rust_database_internals::indexes::{
    IndexEntries, IndexEntryKey, IndexUniqueness, PrimaryKeyValue,
};

const ROWS: usize = 10_000;
const LOOKUP_ROUNDS: usize = 100;

fn main() {
    let results = [
        benchmark_unique_insert(),
        benchmark_non_unique_lookup(),
        benchmark_selectivity_calculation(),
    ];

    println!("\nÍndices benchmark educativo");
    println!("Modelo: entradas únicas, no únicas y selectividad");
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

fn benchmark_unique_insert() -> BenchmarkResult {
    let mut index = IndexEntries::new(IndexUniqueness::Unique);
    let start = Instant::now();

    for row in 0..ROWS {
        index
            .insert(key_for("email", row), primary_key_for(row))
            .expect("las llaves generadas son únicas");
    }

    black_box(index);

    BenchmarkResult {
        name: "insert índice único",
        operations: ROWS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_non_unique_lookup() -> BenchmarkResult {
    let index = build_country_index();
    let lookup_key = IndexEntryKey::from("country-7");
    let start = Instant::now();

    for _ in 0..LOOKUP_ROUNDS {
        black_box(index.primary_keys_for(&lookup_key));
    }

    BenchmarkResult {
        name: "lookup índice no único",
        operations: LOOKUP_ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_selectivity_calculation() -> BenchmarkResult {
    let index = build_country_index();
    let start = Instant::now();

    for _ in 0..LOOKUP_ROUNDS {
        black_box(index.selectivity());
    }

    BenchmarkResult {
        name: "calcular selectividad",
        operations: LOOKUP_ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn build_country_index() -> IndexEntries {
    let mut index = IndexEntries::new(IndexUniqueness::NonUnique);

    for row in 0..ROWS {
        index
            .insert(key_for("country", row % 25), primary_key_for(row))
            .expect("el índice no único acepta valores repetidos");
    }

    index
}

fn key_for(prefix: &str, value: usize) -> IndexEntryKey {
    IndexEntryKey::from(format!("{prefix}-{value}").as_str())
}

fn primary_key_for(row: usize) -> PrimaryKeyValue {
    PrimaryKeyValue::from(format!("customer-{row}").as_str())
}
