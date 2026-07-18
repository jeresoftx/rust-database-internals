use rust_database_internals::btree::{BTree, Key};

fn main() {
    let tree = BTree::new(3).expect("max_keys_per_node=3 debe crear un B-Tree válido");

    assert!(tree.is_empty());
    assert_eq!(tree.len(), 0);
    assert_eq!(tree.height(), 0);
    assert_eq!(tree.search(Key::new(42)), Ok(None));

    println!("Árbol vacío: len={}, height={}", tree.len(), tree.height());
}
