use rust_database_internals::btree::{BTree, Key};

#[test]
fn search_in_empty_tree_returns_none() {
    let tree = match BTree::new(3) {
        Ok(tree) => tree,
        Err(_) => panic!("max_keys_per_node=3 debe crear un B-Tree válido"),
    };

    assert!(tree.is_empty());
    assert_eq!(tree.len(), 0);
    assert_eq!(tree.height(), 0);

    let result = tree.search(Key::new(42));

    assert!(
        matches!(result, Ok(None)),
        "buscar una clave ausente en un B-Tree vacío debe devolver Ok(None)"
    );
}
