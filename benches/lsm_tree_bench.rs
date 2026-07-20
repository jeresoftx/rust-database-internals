use std::hint::black_box;
use std::time::{Duration, Instant};

use rust_database_internals::lsm_tree::{
    CompactionPlan, LsmKey, LsmTree, LsmValue, MemTable, SegmentId,
};

const ROWS: usize = 10_000;
const SEARCH_ROUNDS: usize = 100_000;

fn main() {
    let results = [
        benchmark_memtable_writes(),
        benchmark_search_across_segments(),
        benchmark_flush_to_sstable(),
        benchmark_compaction(),
    ];

    println!("\nLSM Tree benchmark educativo");
    println!("Modelo: MemTable, búsqueda, flush y compaction");
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

fn benchmark_memtable_writes() -> BenchmarkResult {
    let mut memtable = MemTable::new(ROWS).expect("capacidad válida");
    let start = Instant::now();

    for row in 0..ROWS {
        memtable
            .write(LsmKey::new(row as u64), value_for(row))
            .expect("cada escritura debe caber");
    }

    black_box(memtable);

    BenchmarkResult {
        name: "write en MemTable",
        operations: ROWS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_search_across_segments() -> BenchmarkResult {
    let tree = build_tree_with_segments();
    let start = Instant::now();

    for round in 0..SEARCH_ROUNDS {
        black_box(tree.search(LsmKey::new((round % ROWS) as u64)));
    }

    BenchmarkResult {
        name: "search con segmentos",
        operations: SEARCH_ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_flush_to_sstable() -> BenchmarkResult {
    let mut memtable = MemTable::new(ROWS).expect("capacidad válida");
    for row in 0..ROWS {
        memtable
            .write(LsmKey::new(row as u64), value_for(row))
            .expect("cada escritura debe caber");
    }

    let start = Instant::now();
    let sstable = memtable
        .flush_to_sstable(SegmentId::new(1))
        .expect("flush con datos válido");
    black_box(sstable);

    BenchmarkResult {
        name: "flush a SSTable",
        operations: ROWS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_compaction() -> BenchmarkResult {
    let mut tree = build_tree_with_segments();
    let plan = CompactionPlan::new(
        vec![SegmentId::new(1), SegmentId::new(2)],
        SegmentId::new(3),
    )
    .expect("plan válido");
    let start = Instant::now();

    tree.compact(plan).expect("compaction válida");
    black_box(tree);

    BenchmarkResult {
        name: "compactar segmentos",
        operations: ROWS,
        elapsed: start.elapsed(),
    }
}

fn build_tree_with_segments() -> LsmTree {
    let mut tree = LsmTree::new(ROWS).expect("capacidad válida");

    for row in 0..ROWS {
        tree.write(LsmKey::new(row as u64), value_for(row))
            .expect("cada escritura debe caber");
    }
    tree.flush_to_sstable(SegmentId::new(1))
        .expect("primer flush válido");

    for row in 0..ROWS {
        tree.write(LsmKey::new(row as u64), value_for(row + ROWS))
            .expect("cada escritura debe caber");
    }
    tree.flush_to_sstable(SegmentId::new(2))
        .expect("segundo flush válido");

    tree
}

fn value_for(row: usize) -> LsmValue {
    LsmValue::from(format!("value-{row}").as_str())
}
