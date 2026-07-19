use rust_database_internals::btree::{BTree, Key, RecordPointer};

fn main() {
    let mut tree = BTree::new(3).expect("max_keys_per_node=3 debe crear un B-Tree válido");
    let pointer = RecordPointer {
        page_id: 3,
        slot_id: 7,
    };

    tree.insert(Key::new(70), pointer)
        .expect("insertar una clave única debe funcionar");

    assert_eq!(tree.search(Key::new(70)), Ok(Some(pointer)));
    assert_eq!(tree.search(Key::new(71)), Ok(None));

    println!("Ejercicio búsqueda: Key(70) -> {:?}", pointer);
}
