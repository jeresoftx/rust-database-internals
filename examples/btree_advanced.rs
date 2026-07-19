use rust_database_internals::btree::{BTree, Key, RecordPointer};

fn pointer(slot_id: u16) -> RecordPointer {
    RecordPointer {
        page_id: 1,
        slot_id,
    }
}

fn main() {
    let mut tree = BTree::new(3).expect("max_keys_per_node=3 debe crear un B-Tree válido");

    tree.insert(Key::new(10), pointer(10))
        .expect("10 debe insertarse antes del split");
    tree.insert(Key::new(20), pointer(20))
        .expect("20 debe insertarse antes del split");
    tree.insert(Key::new(30), pointer(30))
        .expect("30 debe insertarse antes del split");
    tree.insert(Key::new(40), pointer(40))
        .expect("40 debe disparar el primer split de raíz");

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
        "Split educativo: separator={:?}, leaves={:?}",
        tree.root_separator(),
        tree.leaf_keys()
    );
}
