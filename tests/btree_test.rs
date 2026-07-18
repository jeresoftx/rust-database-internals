use rust_database_internals::btree::{BTree, BTreeError, Key, RecordPointer};

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

#[test]
fn inserts_without_split_keep_keys_searchable() {
    let mut tree = match BTree::new(3) {
        Ok(tree) => tree,
        Err(_) => panic!("max_keys_per_node=3 debe crear un B-Tree válido"),
    };
    let first = RecordPointer {
        page_id: 1,
        slot_id: 1,
    };
    let second = RecordPointer {
        page_id: 1,
        slot_id: 2,
    };
    let third = RecordPointer {
        page_id: 1,
        slot_id: 3,
    };

    assert_eq!(tree.insert(Key::new(20), second), Ok(()));
    assert_eq!(tree.insert(Key::new(10), first), Ok(()));
    assert_eq!(tree.insert(Key::new(30), third), Ok(()));

    assert_eq!(tree.len(), 3);
    assert_eq!(tree.height(), 1);
    assert_eq!(tree.search(Key::new(10)), Ok(Some(first)));
    assert_eq!(tree.search(Key::new(20)), Ok(Some(second)));
    assert_eq!(tree.search(Key::new(30)), Ok(Some(third)));
}

#[test]
fn duplicate_insert_returns_clear_error() {
    let mut tree = match BTree::new(3) {
        Ok(tree) => tree,
        Err(_) => panic!("max_keys_per_node=3 debe crear un B-Tree válido"),
    };
    let pointer = RecordPointer {
        page_id: 7,
        slot_id: 2,
    };

    assert_eq!(tree.insert(Key::new(42), pointer), Ok(()));

    assert_eq!(
        tree.insert(Key::new(42), pointer),
        Err(BTreeError::DuplicateKey(Key::new(42)))
    );
    assert_eq!(tree.len(), 1);
    assert_eq!(tree.search(Key::new(42)), Ok(Some(pointer)));
}

#[test]
fn insert_without_split_reports_full_root() {
    let mut tree = match BTree::new(3) {
        Ok(tree) => tree,
        Err(_) => panic!("max_keys_per_node=3 debe crear un B-Tree válido"),
    };
    let pointer = RecordPointer {
        page_id: 1,
        slot_id: 1,
    };

    assert_eq!(tree.insert(Key::new(10), pointer), Ok(()));
    assert_eq!(tree.insert(Key::new(20), pointer), Ok(()));
    assert_eq!(tree.insert(Key::new(30), pointer), Ok(()));

    assert_eq!(
        tree.insert(Key::new(40), pointer),
        Err(BTreeError::NodeFullRequiresSplit {
            max_keys_per_node: 3
        })
    );
    assert_eq!(tree.len(), 3);
    assert_eq!(tree.search(Key::new(40)), Ok(None));
}

#[test]
fn inserting_past_root_capacity_splits_root_and_preserves_search() {
    let mut tree = match BTree::new(3) {
        Ok(tree) => tree,
        Err(_) => panic!("max_keys_per_node=3 debe crear un B-Tree válido"),
    };
    let ten = RecordPointer {
        page_id: 1,
        slot_id: 10,
    };
    let twenty = RecordPointer {
        page_id: 1,
        slot_id: 20,
    };
    let thirty = RecordPointer {
        page_id: 1,
        slot_id: 30,
    };
    let forty = RecordPointer {
        page_id: 1,
        slot_id: 40,
    };

    assert_eq!(tree.insert(Key::new(10), ten), Ok(()));
    assert_eq!(tree.insert(Key::new(20), twenty), Ok(()));
    assert_eq!(tree.insert(Key::new(30), thirty), Ok(()));
    assert_eq!(tree.insert(Key::new(40), forty), Ok(()));

    assert_eq!(tree.len(), 4);
    assert_eq!(tree.height(), 2);
    assert_eq!(tree.search(Key::new(10)), Ok(Some(ten)));
    assert_eq!(tree.search(Key::new(20)), Ok(Some(twenty)));
    assert_eq!(tree.search(Key::new(30)), Ok(Some(thirty)));
    assert_eq!(tree.search(Key::new(40)), Ok(Some(forty)));
    assert_eq!(tree.search(Key::new(99)), Ok(None));
}
