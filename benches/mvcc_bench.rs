use std::hint::black_box;
use std::time::{Duration, Instant};

use rust_database_internals::mvcc::{
    LogicalTimestamp, RecordId, RecordValue, RecordVersion, Snapshot, VersionChain, VersionId,
    VisibilityDecision,
};

const ROUNDS: usize = 10_000;
const VERSIONS_PER_CHAIN: u64 = 128;

fn main() {
    let results = [
        benchmark_append_versions(),
        benchmark_snapshot_reads(),
        benchmark_visibility_decisions(),
        benchmark_logical_deletes(),
    ];

    println!("\nMVCC benchmark educativo");
    println!("Modelo: cadenas de versiones, snapshot reads y decisiones de visibilidad");
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

fn benchmark_append_versions() -> BenchmarkResult {
    let start = Instant::now();

    for round in 0..ROUNDS {
        let mut chain = chain_for(round);

        for timestamp in 1..=VERSIONS_PER_CHAIN {
            chain
                .append(
                    LogicalTimestamp::new(timestamp),
                    value_for("saldo", timestamp),
                )
                .expect("cada timestamp generado debe ser monótono");
        }

        black_box(chain);
    }

    BenchmarkResult {
        name: "append de versiones",
        operations: ROUNDS * VERSIONS_PER_CHAIN as usize,
        elapsed: start.elapsed(),
    }
}

fn benchmark_snapshot_reads() -> BenchmarkResult {
    let chain = populated_chain();
    let start = Instant::now();

    for round in 0..ROUNDS {
        let timestamp = LogicalTimestamp::new((round as u64 % VERSIONS_PER_CHAIN) + 1);
        let snapshot = Snapshot::new(timestamp);
        black_box(chain.read(&snapshot));
    }

    black_box(chain);

    BenchmarkResult {
        name: "snapshot read",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_visibility_decisions() -> BenchmarkResult {
    let record_id = RecordId::new("accounts/visibility").expect("id válido");
    let mut version = RecordVersion::new(
        record_id,
        VersionId::new(1),
        LogicalTimestamp::new(10),
        RecordValue::new("saldo=100").expect("valor válido"),
    );

    version
        .delete_at(LogicalTimestamp::new(90))
        .expect("cerrar la versión debe ser válido");

    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut visible = 0usize;

        for timestamp in 1..=128 {
            let decision = version.visibility_at(LogicalTimestamp::new(timestamp));

            if matches!(decision, VisibilityDecision::Visible) {
                visible += 1;
            }
        }

        black_box(visible);
    }

    BenchmarkResult {
        name: "barrido de visibilidad",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_logical_deletes() -> BenchmarkResult {
    let start = Instant::now();

    for round in 0..ROUNDS {
        let mut chain = chain_for(round);
        chain
            .append(
                LogicalTimestamp::new(10),
                RecordValue::new("saldo=100").expect("valor válido"),
            )
            .expect("la primera versión debe entrar");
        chain
            .delete_latest_at(LogicalTimestamp::new(12))
            .expect("cerrar la versión debe ser válido");
        black_box(chain);
    }

    BenchmarkResult {
        name: "borrado lógico",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn populated_chain() -> VersionChain {
    let mut chain = VersionChain::new(RecordId::new("accounts/hot-row").expect("id válido"));

    for timestamp in 1..=VERSIONS_PER_CHAIN {
        chain
            .append(
                LogicalTimestamp::new(timestamp),
                value_for("saldo", timestamp),
            )
            .expect("cada timestamp generado debe ser monótono");
    }

    chain
}

fn chain_for(round: usize) -> VersionChain {
    VersionChain::new(RecordId::new(format!("accounts/{round}")).expect("id válido"))
}

fn value_for(prefix: &str, value: u64) -> RecordValue {
    RecordValue::new(format!("{prefix}={value}")).expect("valor válido")
}
