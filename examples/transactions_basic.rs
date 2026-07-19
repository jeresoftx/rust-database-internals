use rust_database_internals::transactions::{TransactionManager, TransactionState};

fn main() {
    let mut manager = TransactionManager::new();
    let transaction_id = manager
        .begin()
        .expect("abrir una transacción debe producir un id");

    assert_eq!(
        manager.state(transaction_id),
        Some(TransactionState::Active)
    );

    manager
        .commit(transaction_id)
        .expect("commit debe cerrar una transacción activa");

    assert_eq!(
        manager.state(transaction_id),
        Some(TransactionState::Committed)
    );

    println!(
        "La transacción {} pasó de active a committed.",
        transaction_id.value()
    );
}
