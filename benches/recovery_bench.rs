use std::hint::black_box;
use std::time::{Duration, Instant};

use rust_database_internals::{
    recovery::RecoveryPlan,
    wal::{LogOperation, PageId, PageImage, PageStore, WalTransactionId, WriteAheadLog},
};

const ROUNDS: usize = 10_000;
const TRANSACTIONS_PER_LOG: usize = 64;

fn main() {
    let results = [
        benchmark_plan_analysis(),
        benchmark_replay_redo(),
        benchmark_replay_undo(),
        benchmark_replay_mixed(),
    ];

    println!("\nRecovery benchmark educativo");
    println!("Modelo: análisis de WAL, redo forward y undo backward");
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

fn benchmark_plan_analysis() -> BenchmarkResult {
    let log = mixed_log();
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let plan = RecoveryPlan::from_wal(&log);
        black_box(plan);
    }

    BenchmarkResult {
        name: "analizar WAL",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_replay_redo() -> BenchmarkResult {
    let log = committed_log();
    let plan = RecoveryPlan::from_wal(&log);
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut store = initial_store("saldo=0");
        let report = plan
            .replay(&log, &mut store)
            .expect("replay de transacciones confirmadas debe ser válido");
        black_box((store, report));
    }

    BenchmarkResult {
        name: "replay redo",
        operations: ROUNDS * TRANSACTIONS_PER_LOG,
        elapsed: start.elapsed(),
    }
}

fn benchmark_replay_undo() -> BenchmarkResult {
    let log = uncommitted_log();
    let plan = RecoveryPlan::from_wal(&log);
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut store = initial_store("saldo=64");
        let report = plan
            .replay(&log, &mut store)
            .expect("replay de transacciones incompletas debe ser válido");
        black_box((store, report));
    }

    BenchmarkResult {
        name: "replay undo",
        operations: ROUNDS * TRANSACTIONS_PER_LOG,
        elapsed: start.elapsed(),
    }
}

fn benchmark_replay_mixed() -> BenchmarkResult {
    let log = mixed_log();
    let plan = RecoveryPlan::from_wal(&log);
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut store = initial_store("saldo=64");
        let report = plan
            .replay(&log, &mut store)
            .expect("replay mixto debe ser válido");
        black_box((store, report));
    }

    BenchmarkResult {
        name: "replay mixto",
        operations: ROUNDS * TRANSACTIONS_PER_LOG,
        elapsed: start.elapsed(),
    }
}

fn committed_log() -> WriteAheadLog {
    let mut log = WriteAheadLog::new();

    for transaction in 1..=TRANSACTIONS_PER_LOG {
        let transaction_id = WalTransactionId::new(transaction as u64);
        log.append_begin(transaction_id);
        log.append(transaction_id, account_update(transaction - 1, transaction));
        log.append_commit(transaction_id);
    }

    log
}

fn uncommitted_log() -> WriteAheadLog {
    let mut log = WriteAheadLog::new();

    for transaction in 1..=TRANSACTIONS_PER_LOG {
        let transaction_id = WalTransactionId::new(transaction as u64);
        log.append_begin(transaction_id);
        log.append(transaction_id, account_update(transaction - 1, transaction));
    }

    log
}

fn mixed_log() -> WriteAheadLog {
    let mut log = WriteAheadLog::new();

    for transaction in 1..=TRANSACTIONS_PER_LOG {
        let transaction_id = WalTransactionId::new(transaction as u64);
        log.append_begin(transaction_id);
        log.append(transaction_id, account_update(transaction - 1, transaction));

        if transaction % 2 == 0 {
            log.append_commit(transaction_id);
        }
    }

    log
}

fn initial_store(value: &str) -> PageStore {
    let mut store = PageStore::new();
    store.write(
        PageId::new("heap/accounts/0001").expect("page id válido"),
        PageImage::new(value).expect("imagen válida"),
    );
    store
}

fn account_update(before: usize, after: usize) -> LogOperation {
    LogOperation::update(
        PageId::new("heap/accounts/0001").expect("page id válido"),
        PageImage::new(format!("saldo={before}")).expect("imagen before válida"),
        PageImage::new(format!("saldo={after}")).expect("imagen after válida"),
    )
    .expect("el cambio generado debe ser observable")
}
