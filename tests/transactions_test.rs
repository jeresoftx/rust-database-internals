use rust_database_internals::transactions::{
    TransactionError, TransactionId, TransactionManager, TransactionState,
};

#[test]
fn transaction_id_exposes_logical_value() {
    let transaction_id = TransactionId::new(42);

    assert_eq!(transaction_id.value(), 42);
}

#[test]
fn transaction_manager_starts_empty() {
    let manager = TransactionManager::new();

    assert!(manager.is_empty());
    assert_eq!(manager.len(), 0);
    assert_eq!(manager.next_transaction_id(), TransactionId::new(1));
}

#[test]
fn transaction_manager_can_register_initial_transaction_state() {
    let mut manager = TransactionManager::new();

    let transaction_id = manager
        .register(TransactionState::Active)
        .expect("registrar una transacción activa debe producir un id");

    assert_eq!(transaction_id, TransactionId::new(1));
    assert!(!manager.is_empty());
    assert_eq!(manager.len(), 1);
    assert_eq!(manager.next_transaction_id(), TransactionId::new(2));
    assert_eq!(
        manager.state(transaction_id),
        Some(TransactionState::Active)
    );
}

#[test]
fn begin_registers_an_active_transaction() {
    let mut manager = TransactionManager::new();

    let transaction_id = manager
        .begin()
        .expect("abrir una transacción debe producir un id");

    assert_eq!(transaction_id, TransactionId::new(1));
    assert_eq!(manager.len(), 1);
    assert_eq!(manager.next_transaction_id(), TransactionId::new(2));
    assert_eq!(
        manager.state(transaction_id),
        Some(TransactionState::Active)
    );
}

#[test]
fn commit_moves_active_transaction_to_committed() {
    let mut manager = TransactionManager::new();
    let transaction_id = manager
        .begin()
        .expect("abrir una transacción debe producir un id");

    manager
        .commit(transaction_id)
        .expect("commit de una transacción activa debe cerrar con éxito");

    assert_eq!(
        manager.state(transaction_id),
        Some(TransactionState::Committed)
    );
}

#[test]
fn rollback_moves_active_transaction_to_rolled_back() {
    let mut manager = TransactionManager::new();
    let transaction_id = manager
        .begin()
        .expect("abrir una transacción debe producir un id");

    manager
        .rollback(transaction_id)
        .expect("rollback de una transacción activa debe cerrar con éxito");

    assert_eq!(
        manager.state(transaction_id),
        Some(TransactionState::RolledBack)
    );
}

#[test]
fn transaction_manager_returns_none_for_unknown_transaction() {
    let manager = TransactionManager::new();

    assert_eq!(manager.state(TransactionId::new(404)), None);
}

#[test]
fn commit_rejects_unknown_transaction() {
    let mut manager = TransactionManager::new();
    let unknown_transaction = TransactionId::new(404);

    assert_eq!(
        manager.commit(unknown_transaction),
        Err(TransactionError::UnknownTransaction(unknown_transaction))
    );
}

#[test]
fn rollback_rejects_unknown_transaction() {
    let mut manager = TransactionManager::new();
    let unknown_transaction = TransactionId::new(404);

    assert_eq!(
        manager.rollback(unknown_transaction),
        Err(TransactionError::UnknownTransaction(unknown_transaction))
    );
}

#[test]
fn commit_rejects_transaction_that_is_already_committed() {
    let mut manager = TransactionManager::new();
    let transaction_id = manager
        .begin()
        .expect("abrir una transacción debe producir un id");
    manager
        .commit(transaction_id)
        .expect("primer commit debe cerrar la transacción");

    assert_eq!(
        manager.commit(transaction_id),
        Err(TransactionError::InvalidStateTransition {
            transaction_id,
            from: TransactionState::Committed,
            requested: TransactionState::Committed,
        })
    );
}

#[test]
fn rollback_rejects_transaction_that_is_already_committed() {
    let mut manager = TransactionManager::new();
    let transaction_id = manager
        .begin()
        .expect("abrir una transacción debe producir un id");
    manager
        .commit(transaction_id)
        .expect("commit debe cerrar la transacción");

    assert_eq!(
        manager.rollback(transaction_id),
        Err(TransactionError::InvalidStateTransition {
            transaction_id,
            from: TransactionState::Committed,
            requested: TransactionState::RolledBack,
        })
    );
}

#[test]
fn commit_rejects_transaction_that_is_already_rolled_back() {
    let mut manager = TransactionManager::new();
    let transaction_id = manager
        .begin()
        .expect("abrir una transacción debe producir un id");
    manager
        .rollback(transaction_id)
        .expect("rollback debe cerrar la transacción");

    assert_eq!(
        manager.commit(transaction_id),
        Err(TransactionError::InvalidStateTransition {
            transaction_id,
            from: TransactionState::RolledBack,
            requested: TransactionState::Committed,
        })
    );
}

#[test]
fn transaction_states_name_the_lifecycle() {
    assert_eq!(TransactionState::Active.as_str(), "active");
    assert_eq!(TransactionState::Committed.as_str(), "committed");
    assert_eq!(TransactionState::RolledBack.as_str(), "rolled_back");
}
