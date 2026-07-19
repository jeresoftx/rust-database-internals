use std::hint::black_box;
use std::time::{Duration, Instant};

use rust_database_internals::acid::{
    AtomicUnit, Change, CommitLog, OperationId, ReadCommittedCell, UniqueConstraint,
};

const ROUNDS: usize = 10_000;

fn main() {
    let results = [
        benchmark_atomicity_rollback(),
        benchmark_consistency_checks(),
        benchmark_isolation_pending_writes(),
        benchmark_durability_sync(),
    ];

    println!("\nACID benchmark educativo");
    println!("Modelo: fallas parciales, constraints, aislamiento y durabilidad");
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

fn benchmark_atomicity_rollback() -> BenchmarkResult {
    let start = Instant::now();

    for round in 0..ROUNDS {
        let mut unit = AtomicUnit::new();
        unit.stage(change_for("debit", round))
            .expect("el débito debe prepararse");
        unit.stage(change_for("credit", round))
            .expect("el crédito debe prepararse");
        unit.rollback()
            .expect("rollback debe descartar la unidad completa");
        black_box(unit);
    }

    BenchmarkResult {
        name: "rollback de falla parcial",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_consistency_checks() -> BenchmarkResult {
    let mut constraint =
        UniqueConstraint::new("customers.email").expect("la constraint generada debe tener nombre");
    let start = Instant::now();

    for round in 0..ROUNDS {
        constraint
            .insert(format!("customer-{round}@example.com"))
            .expect("cada correo generado debe ser único");
    }

    black_box(constraint);

    BenchmarkResult {
        name: "validar unicidad",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_isolation_pending_writes() -> BenchmarkResult {
    let start = Instant::now();

    for round in 0..ROUNDS {
        let mut cell = ReadCommittedCell::new("balance=100");
        let writer = OperationId::new(round as u64 + 1);
        cell.write_pending(writer, "balance=50")
            .expect("el escritor debe preparar el cambio");
        black_box(cell.read_committed());
        cell.commit_pending(writer)
            .expect("el escritor debe publicar el cambio");
        black_box(cell);
    }

    BenchmarkResult {
        name: "lectura confirmada",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_durability_sync() -> BenchmarkResult {
    let mut log = CommitLog::new();
    let start = Instant::now();

    for round in 0..ROUNDS {
        let commit_id = log
            .append_commit(format!("commit-{round}"))
            .expect("el commit generado debe registrarse");
        log.sync(commit_id)
            .expect("sync debe marcar durable un commit conocido");
    }

    black_box(log);

    BenchmarkResult {
        name: "append + sync",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn change_for(prefix: &str, value: usize) -> Change {
    Change::from(format!("{prefix}-{value}").as_str())
}
