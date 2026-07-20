use rust_database_internals::lsm_tree::{LsmKey, LsmTree, LsmValue};

fn main() {
    let mut tree = LsmTree::new(4).expect("la capacidad debe ser válida");

    tree.write(LsmKey::new(10), LsmValue::from("customer-10"))
        .expect("la escritura debe caber en memoria");

    assert_eq!(
        tree.search(LsmKey::new(10)),
        Some(LsmValue::from("customer-10"))
    );
    assert_eq!(tree.search(LsmKey::new(99)), None);

    println!("LSM básico: escritura reciente visible desde MemTable");
}
