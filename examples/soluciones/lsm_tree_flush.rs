use rust_database_internals::lsm_tree::{LsmKey, LsmValue, MemTable, SegmentId};

fn main() {
    let mut memtable = MemTable::new(4).expect("la capacidad debe ser válida");

    memtable
        .write(LsmKey::new(30), LsmValue::from("thirty"))
        .expect("la escritura debe caber");
    memtable
        .write(LsmKey::new(10), LsmValue::from("ten"))
        .expect("la escritura debe caber");

    let sstable = memtable
        .flush_to_sstable(SegmentId::new(7))
        .expect("flush con datos debe crear SSTable");

    assert!(memtable.is_empty());
    assert_eq!(sstable.segment_id(), SegmentId::new(7));
    assert_eq!(
        sstable.entries(),
        vec![
            (LsmKey::new(10), LsmValue::from("ten")),
            (LsmKey::new(30), LsmValue::from("thirty")),
        ]
    );

    println!("solución nivel 1: flush ordena y congela la MemTable");
}
