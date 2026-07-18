use rust_database_internals::btree::{BTree, Key, RecordPointer};

fn main() {
    let mut tree = BTree::new(3).expect("max_keys_per_node=3 debe crear un B-Tree válido");
    let customer_pointer = RecordPointer {
        page_id: 12,
        slot_id: 4,
    };

    tree.insert(Key::new(1001), customer_pointer)
        .expect("la primera inserción debe caber en la raíz hoja");

    assert_eq!(tree.len(), 1);
    assert_eq!(tree.height(), 1);
    assert_eq!(tree.search(Key::new(1001)), Ok(Some(customer_pointer)));
    assert_eq!(tree.search(Key::new(9999)), Ok(None));

    println!(
        "Clave 1001 encontrada en page_id={} slot_id={}",
        customer_pointer.page_id, customer_pointer.slot_id
    );
}
