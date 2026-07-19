use rust_database_internals::btree::{BTree, Key, RecordPointer};

fn pointer(slot_id: u16) -> RecordPointer {
    RecordPointer {
        page_id: 4,
        slot_id,
    }
}

fn main() {
    let mut tree = BTree::new(3).expect("max_keys_per_node=3 debe crear un B-Tree válido");

    for key in [10, 20, 30, 40] {
        tree.insert(Key::new(key), pointer(key as u16))
            .expect("el primer overflow debe disparar split de raíz");
    }

    assert_eq!(tree.len(), 4);
    assert_eq!(tree.height(), 2);
    assert_eq!(tree.root_separator(), Some(Key::new(30)));
    assert_eq!(
        tree.leaf_keys(),
        vec![
            vec![Key::new(10), Key::new(20)],
            vec![Key::new(30), Key::new(40)]
        ]
    );
    assert_eq!(tree.search(Key::new(40)), Ok(Some(pointer(40))));

    println!(
        "Ejercicio split: separator={:?}, hojas={:?}",
        tree.root_separator(),
        tree.leaf_keys()
    );
}
