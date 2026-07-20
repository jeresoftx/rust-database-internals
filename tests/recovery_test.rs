use rust_database_internals::{
    recovery::RecoveryPlan,
    wal::{LogOperation, PageId, PageImage, WalTransactionId, WriteAheadLog},
};

#[test]
fn empty_wal_produces_empty_recovery_plan() {
    let log = WriteAheadLog::new();

    let plan = RecoveryPlan::from_wal(&log);

    assert!(plan.is_empty());
    assert!(plan.redo_transactions().is_empty());
    assert!(plan.undo_transactions().is_empty());
}

#[test]
fn crash_before_commit_requires_undo_for_dirty_transaction() {
    let mut log = WriteAheadLog::new();
    let transaction_id = WalTransactionId::new(10);

    log.append_begin(transaction_id);
    log.append(transaction_id, account_update("saldo=100", "saldo=120"));

    let plan = RecoveryPlan::from_wal(&log);

    assert!(plan.requires_undo(transaction_id));
    assert!(!plan.requires_redo(transaction_id));
    assert_eq!(plan.undo_transactions(), &[transaction_id]);
}

#[test]
fn crash_after_commit_requires_redo_for_committed_transaction() {
    let mut log = WriteAheadLog::new();
    let transaction_id = WalTransactionId::new(10);

    log.append_begin(transaction_id);
    log.append(transaction_id, account_update("saldo=100", "saldo=120"));
    log.append_commit(transaction_id);

    let plan = RecoveryPlan::from_wal(&log);

    assert!(plan.requires_redo(transaction_id));
    assert!(!plan.requires_undo(transaction_id));
    assert_eq!(plan.redo_transactions(), &[transaction_id]);
}

#[test]
fn clean_transaction_open_at_crash_does_not_need_undo() {
    let mut log = WriteAheadLog::new();
    let transaction_id = WalTransactionId::new(20);

    log.append_begin(transaction_id);

    let plan = RecoveryPlan::from_wal(&log);

    assert!(plan.is_empty());
    assert!(!plan.requires_undo(transaction_id));
}

#[test]
fn rolled_back_transaction_is_not_redone_or_undone_again() {
    let mut log = WriteAheadLog::new();
    let transaction_id = WalTransactionId::new(30);

    log.append_begin(transaction_id);
    log.append(transaction_id, account_update("saldo=100", "saldo=80"));
    log.append_rollback(transaction_id);

    let plan = RecoveryPlan::from_wal(&log);

    assert!(plan.is_empty());
    assert!(!plan.requires_redo(transaction_id));
    assert!(!plan.requires_undo(transaction_id));
}

fn account_update(before: &str, after: &str) -> LogOperation {
    LogOperation::update(
        PageId::new("heap/accounts/0001").expect("page id válido"),
        PageImage::new(before).expect("imagen before válida"),
        PageImage::new(after).expect("imagen after válida"),
    )
    .expect("update válido")
}
