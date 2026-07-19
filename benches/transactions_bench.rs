use std::hint::black_box;
use std::time::{Duration, Instant};

use rust_database_internals::transactions::{ResourceId, TransactionError, TransactionManager};

const TRANSACTIONS: usize = 10_000;
const CONFLICT_ROUNDS: usize = 10_000;

fn main() {
    let results = [
        benchmark_begin_commit(),
        benchmark_exclusive_locks(),
        benchmark_conflict_detection(),
    ];

    println!("\nTransacciones benchmark educativo");
    println!("Modelo: ciclo de vida, locks exclusivos y conflictos simples");
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

fn benchmark_begin_commit() -> BenchmarkResult {
    let mut manager = TransactionManager::new();
    let start = Instant::now();

    for _ in 0..TRANSACTIONS {
        let transaction_id = manager
            .begin()
            .expect("abrir una transacción debe producir un id");
        manager
            .commit(transaction_id)
            .expect("commit debe cerrar una transacción activa");
    }

    black_box(manager);

    BenchmarkResult {
        name: "begin + commit",
        operations: TRANSACTIONS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_exclusive_locks() -> BenchmarkResult {
    let mut manager = TransactionManager::new();
    let start = Instant::now();

    for row in 0..TRANSACTIONS {
        let transaction_id = manager
            .begin()
            .expect("abrir una transacción debe producir un id");
        manager
            .lock_exclusive(transaction_id, resource_for("accounts", row))
            .expect("cada transacción toma un recurso distinto");
    }

    black_box(manager);

    BenchmarkResult {
        name: "lock exclusivo sin conflicto",
        operations: TRANSACTIONS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_conflict_detection() -> BenchmarkResult {
    let mut manager = TransactionManager::new();
    let holder = manager
        .begin()
        .expect("abrir la transacción dueña debe producir un id");
    let resource = ResourceId::new("accounts/hot-row").expect("recurso válido");

    manager
        .lock_exclusive(holder, resource.clone())
        .expect("la transacción dueña toma el recurso");

    let start = Instant::now();

    for _ in 0..CONFLICT_ROUNDS {
        let requester = manager
            .begin()
            .expect("abrir una transacción solicitante debe producir un id");
        assert!(matches!(
            manager.lock_exclusive(requester, resource.clone()),
            Err(TransactionError::ResourceConflict { .. })
        ));
    }

    black_box(manager);

    BenchmarkResult {
        name: "detectar conflicto",
        operations: CONFLICT_ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn resource_for(prefix: &str, value: usize) -> ResourceId {
    ResourceId::new(format!("{prefix}/{value}")).expect("el recurso generado es válido")
}
