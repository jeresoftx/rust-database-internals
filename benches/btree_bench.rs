use std::hint::black_box;
use std::time::{Duration, Instant};

use rust_database_internals::btree::{BTree, Key, RecordPointer};

const INSERT_KEYS: usize = 3_000;
const SEARCH_KEYS: usize = 3_000;
const SEARCH_ROUNDS: usize = 100;

fn main() {
    let results = [
        benchmark_insert_leaf(),
        benchmark_search_hits_after_split(),
        benchmark_search_misses_after_split(),
    ];

    println!("\nB-Tree benchmark educativo");
    println!("Modelo: raíz hoja y primer split de raíz");
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

fn benchmark_insert_leaf() -> BenchmarkResult {
    let mut tree =
        BTree::new(INSERT_KEYS + 1).expect("la capacidad debe permitir inserción lineal");
    let start = Instant::now();

    for value in 0..INSERT_KEYS {
        tree.insert(Key::new(value as u64), pointer_for(value))
            .expect("las claves generadas son únicas");
    }

    black_box(tree);

    BenchmarkResult {
        name: "insert raíz hoja",
        operations: INSERT_KEYS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_search_hits_after_split() -> BenchmarkResult {
    let tree = build_split_tree();
    let start = Instant::now();

    for _ in 0..SEARCH_ROUNDS {
        for value in 0..SEARCH_KEYS {
            let pointer = tree
                .search(Key::new(value as u64))
                .expect("buscar no debe fallar")
                .expect("la clave debe existir");
            black_box(pointer);
        }
    }

    BenchmarkResult {
        name: "search hit con split",
        operations: SEARCH_KEYS * SEARCH_ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_search_misses_after_split() -> BenchmarkResult {
    let tree = build_split_tree();
    let start = Instant::now();

    for _ in 0..SEARCH_ROUNDS {
        for value in SEARCH_KEYS..(SEARCH_KEYS * 2) {
            let pointer = tree
                .search(Key::new(value as u64))
                .expect("buscar no debe fallar");
            black_box(pointer);
        }
    }

    BenchmarkResult {
        name: "search miss con split",
        operations: SEARCH_KEYS * SEARCH_ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn build_split_tree() -> BTree {
    let mut tree = BTree::new(2_048).expect("la capacidad debe permitir el primer split");

    for value in 0..SEARCH_KEYS {
        tree.insert(Key::new(value as u64), pointer_for(value))
            .expect("el tamaño elegido cabe en el modelo educativo actual");
    }

    tree
}

fn pointer_for(value: usize) -> RecordPointer {
    RecordPointer {
        page_id: (value / 128) as u64,
        slot_id: (value % 128) as u16,
    }
}
