use rust_database_internals::lsm_tree::{LsmKey, LsmTree, LsmValue, SegmentId};

fn main() {
    let mut tree = LsmTree::new(4).expect("la capacidad debe ser válida");

    tree.write(LsmKey::new(42), LsmValue::from("old"))
        .expect("la escritura debe caber");
    tree.flush_to_sstable(SegmentId::new(1))
        .expect("flush válido");
    tree.write(LsmKey::new(42), LsmValue::from("new"))
        .expect("la versión nueva debe caber");

    assert_eq!(tree.search(LsmKey::new(42)), Some(LsmValue::from("new")));

    println!("solución nivel 2: la versión en MemTable gana");
}
