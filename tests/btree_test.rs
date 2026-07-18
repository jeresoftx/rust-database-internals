use rust_database_internals::btree::{BTree, Key, RecordPointer};

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

#[test]
fn simple_insert_can_be_found_by_search() {
    let mut tree = match BTree::new(3) {
        Ok(tree) => tree,
        Err(_) => panic!("max_keys_per_node=3 debe crear un B-Tree válido"),
    };
    let pointer = RecordPointer {
        page_id: 7,
        slot_id: 2,
    };

    let insert_result = tree.insert(Key::new(42), pointer);

    assert!(insert_result.is_ok());
    assert!(!tree.is_empty());
    assert_eq!(tree.len(), 1);
    assert_eq!(tree.height(), 1);
    assert_eq!(tree.search(Key::new(42)), Ok(Some(pointer)));
    assert_eq!(tree.search(Key::new(9)), Ok(None));
}
