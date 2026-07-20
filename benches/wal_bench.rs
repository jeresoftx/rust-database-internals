use std::hint::black_box;
use std::time::{Duration, Instant};

use rust_database_internals::wal::{
    LogOperation, LogRecord, LogSequenceNumber, PageId, PageImage, PageStore, WalError,
    WalTransactionId, WriteAheadLog,
};

const ROUNDS: usize = 10_000;
const RECORDS_PER_LOG: usize = 128;

fn main() {
    let results = [
        benchmark_append_records(),
        benchmark_redo_updates(),
        benchmark_undo_updates(),
        benchmark_lsn_validation(),
    ];

    println!("\nWrite-Ahead Log benchmark educativo");
    println!("Modelo: append-only log, redo, undo y validación de LSN");
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

fn benchmark_append_records() -> BenchmarkResult {
    let start = Instant::now();

    for round in 0..ROUNDS {
        let mut log = WriteAheadLog::new();
        let transaction_id = WalTransactionId::new(round as u64 + 1);

        for record in 0..RECORDS_PER_LOG {
            log.append(transaction_id, update_for(record));
        }

        black_box(log);
    }

    BenchmarkResult {
        name: "append de registros",
        operations: ROUNDS * RECORDS_PER_LOG,
        elapsed: start.elapsed(),
    }
}

fn benchmark_redo_updates() -> BenchmarkResult {
    let records = update_records();
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut store = PageStore::new();

        for record in &records {
            store
                .redo(record)
                .expect("cada registro update permite redo");
        }

        black_box(store);
    }

    BenchmarkResult {
        name: "redo de updates",
        operations: ROUNDS * records.len(),
        elapsed: start.elapsed(),
    }
}

fn benchmark_undo_updates() -> BenchmarkResult {
    let records = update_records();
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut store = PageStore::new();

        for record in &records {
            store
                .undo(record)
                .expect("cada registro update permite undo");
        }

        black_box(store);
    }

    BenchmarkResult {
        name: "undo de updates",
        operations: ROUNDS * records.len(),
        elapsed: start.elapsed(),
    }
}

fn benchmark_lsn_validation() -> BenchmarkResult {
    let start = Instant::now();

    for round in 0..ROUNDS {
        let mut log = WriteAheadLog::new();
        let record = LogRecord::begin(
            LogSequenceNumber::new(2),
            WalTransactionId::new(round as u64 + 1),
        );

        assert!(matches!(
            log.append_record(record),
            Err(WalError::UnexpectedLsn { .. })
        ));

        black_box(log);
    }

    BenchmarkResult {
        name: "rechazo de LSN inesperado",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn update_records() -> Vec<LogRecord> {
    (0..RECORDS_PER_LOG)
        .map(|record| {
            LogRecord::new(
                LogSequenceNumber::new(record as u64 + 1),
                WalTransactionId::new(10),
                update_for(record),
            )
        })
        .collect()
}

fn update_for(record: usize) -> LogOperation {
    LogOperation::update(
        PageId::new(format!("heap/accounts/{record:04}")).expect("page id válido"),
        PageImage::new(format!("saldo={record}")).expect("imagen before válida"),
        PageImage::new(format!("saldo={}", record + 1)).expect("imagen after válida"),
    )
    .expect("el cambio generado debe ser observable")
}
