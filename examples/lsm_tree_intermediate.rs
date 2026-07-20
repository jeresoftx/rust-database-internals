use rust_database_internals::lsm_tree::{LsmKey, LsmTree, LsmValue, SegmentId};

fn main() {
    let mut tree = LsmTree::new(4).expect("la capacidad debe ser válida");

    tree.write(LsmKey::new(1), LsmValue::from("old"))
        .expect("la escritura debe caber");
    tree.flush_to_sstable(SegmentId::new(1))
        .expect("flush con datos debe producir segmento");

    tree.write(LsmKey::new(1), LsmValue::from("new"))
        .expect("la versión nueva debe caber en memoria");

    assert_eq!(tree.segments().len(), 1);
    assert_eq!(tree.search(LsmKey::new(1)), Some(LsmValue::from("new")));

    println!("LSM intermedio: MemTable tiene precedencia sobre SSTable");
}
