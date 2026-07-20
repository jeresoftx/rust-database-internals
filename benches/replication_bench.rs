use std::hint::black_box;
use std::time::{Duration, Instant};

use rust_database_internals::{
    replication::{ReplicationAckMode, ReplicationCluster, ReplicationDecision, ReplicationNode},
    wal::{LogOperation, PageId, PageImage, WalTransactionId},
};

const ROUNDS: usize = 10_000;
const RECORDS_PER_PRIMARY: usize = 64;

fn main() {
    let results = [
        benchmark_primary_writes(),
        benchmark_replicate_to_replica(),
        benchmark_measure_lag(),
        benchmark_sync_confirmation(),
    ];

    println!("\nReplicación benchmark educativo");
    println!("Modelo: primary/replica, lag y confirmación");
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

fn benchmark_primary_writes() -> BenchmarkResult {
    let start = Instant::now();

    for round in 0..ROUNDS {
        let mut primary = ReplicationNode::primary(format!("primary-{round}"))
            .expect("primary generado debe ser válido");

        for record in 0..RECORDS_PER_PRIMARY {
            primary
                .append_local_update(
                    WalTransactionId::new(record as u64 + 1),
                    account_update(record, record + 1),
                )
                .expect("primary debe aceptar escrituras locales");
        }

        black_box(primary);
    }

    BenchmarkResult {
        name: "escrituras en primary",
        operations: ROUNDS * RECORDS_PER_PRIMARY,
        elapsed: start.elapsed(),
    }
}

fn benchmark_replicate_to_replica() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut cluster = cluster_with_lagged_replica();
        let report = cluster
            .replicate_to("replica-1")
            .expect("copiar WAL hacia réplica debe ser válido");
        black_box((cluster, report));
    }

    BenchmarkResult {
        name: "copiar WAL a réplica",
        operations: ROUNDS * RECORDS_PER_PRIMARY,
        elapsed: start.elapsed(),
    }
}

fn benchmark_measure_lag() -> BenchmarkResult {
    let cluster = cluster_with_lagged_replica();
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let lag = cluster
            .replica_lag("replica-1")
            .expect("la réplica conocida permite medir lag");
        black_box(lag);
    }

    BenchmarkResult {
        name: "medir lag",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_sync_confirmation() -> BenchmarkResult {
    let cluster = cluster_with_lagged_replica();
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let decision = cluster
            .confirm_write(ReplicationAckMode::Sync)
            .expect("confirmación síncrona debe evaluarse");
        assert_eq!(
            decision,
            ReplicationDecision::WaitingForReplicas {
                pending_replicas: 1,
                pending_records: RECORDS_PER_PRIMARY,
            }
        );
        black_box(decision);
    }

    BenchmarkResult {
        name: "confirmación sync con lag",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn cluster_with_lagged_replica() -> ReplicationCluster {
    let mut primary = ReplicationNode::primary("primary-1").expect("primary válido");

    for record in 0..RECORDS_PER_PRIMARY {
        primary
            .append_local_update(
                WalTransactionId::new(record as u64 + 1),
                account_update(record, record + 1),
            )
            .expect("primary debe aceptar escritura");
    }

    let replica = ReplicationNode::replica("replica-1").expect("réplica válida");
    ReplicationCluster::new(primary, vec![replica]).expect("cluster válido")
}

fn account_update(before: usize, after: usize) -> LogOperation {
    LogOperation::update(
        PageId::new("heap/accounts/0001").expect("page id válido"),
        PageImage::new(format!("saldo={before}")).expect("imagen before válida"),
        PageImage::new(format!("saldo={after}")).expect("imagen after válida"),
    )
    .expect("el cambio generado debe ser observable")
}
