use rust_database_internals::transactions::{TransactionManager, TransactionState};

fn main() {
    let mut manager = TransactionManager::new();
    let transaction_id = manager
        .begin()
        .expect("abrir una transacción debe producir un id");

    manager
        .commit(transaction_id)
        .expect("commit debe cerrar una transacción activa");

    assert_eq!(
        manager.state(transaction_id),
        Some(TransactionState::Committed)
    );

    println!("Ejercicio completado: commit deja la transacción en committed.");
}
