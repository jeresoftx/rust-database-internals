use rust_database_internals::btree::{BTree, BTreeError, Key, RecordPointer};

fn pointer(slot_id: u16) -> RecordPointer {
    RecordPointer {
        page_id: 2,
        slot_id,
    }
}

fn main() {
    let mut tree = BTree::new(3).expect("max_keys_per_node=3 debe crear un B-Tree válido");

    tree.insert(Key::new(30), pointer(30))
        .expect("30 debe insertarse");
    tree.insert(Key::new(10), pointer(10))
        .expect("10 debe insertarse antes de 30");
    tree.insert(Key::new(20), pointer(20))
        .expect("20 debe insertarse entre 10 y 30");

    assert_eq!(tree.len(), 3);
    assert_eq!(tree.height(), 1);
    assert_eq!(
        tree.leaf_keys(),
        vec![vec![Key::new(10), Key::new(20), Key::new(30)]]
    );
    assert_eq!(
        tree.insert(Key::new(20), pointer(99)),
        Err(BTreeError::DuplicateKey(Key::new(20)))
    );

    println!("Ejercicio inserción: hojas={:?}", tree.leaf_keys());
}
