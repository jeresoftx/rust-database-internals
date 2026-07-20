use rust_database_internals::{
    recovery::RecoveryPlan,
    wal::{LogOperation, PageId, PageImage, PageStore, WalTransactionId, WriteAheadLog},
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

#[test]
fn replay_redoes_updates_from_committed_transactions() {
    let mut log = WriteAheadLog::new();
    let transaction_id = WalTransactionId::new(10);
    let page_id = account_page();
    let before = page_image("saldo=100");
    let after = page_image("saldo=120");

    log.append_begin(transaction_id);
    log.append(transaction_id, account_update("saldo=100", "saldo=120"));
    log.append_commit(transaction_id);

    let mut store = PageStore::new();
    store.write(page_id.clone(), before);
    let plan = RecoveryPlan::from_wal(&log);

    let report = plan.replay(&log, &mut store).expect("replay válido");

    assert_eq!(store.read(&page_id), Some(&after));
    assert_eq!(report.redone_records(), 1);
    assert_eq!(report.undone_records(), 0);
}

#[test]
fn replay_undoes_updates_from_uncommitted_transactions() {
    let mut log = WriteAheadLog::new();
    let transaction_id = WalTransactionId::new(20);
    let page_id = account_page();
    let before = page_image("saldo=100");
    let after = page_image("saldo=130");

    log.append_begin(transaction_id);
    log.append(transaction_id, account_update("saldo=100", "saldo=130"));

    let mut store = PageStore::new();
    store.write(page_id.clone(), after);
    let plan = RecoveryPlan::from_wal(&log);

    let report = plan.replay(&log, &mut store).expect("replay válido");

    assert_eq!(store.read(&page_id), Some(&before));
    assert_eq!(report.redone_records(), 0);
    assert_eq!(report.undone_records(), 1);
}

#[test]
fn replay_undoes_uncommitted_updates_in_reverse_wal_order() {
    let mut log = WriteAheadLog::new();
    let transaction_id = WalTransactionId::new(30);
    let page_id = account_page();
    let original = page_image("saldo=100");
    let final_dirty = page_image("saldo=140");

    log.append_begin(transaction_id);
    log.append(transaction_id, account_update("saldo=100", "saldo=120"));
    log.append(transaction_id, account_update("saldo=120", "saldo=140"));

    let mut store = PageStore::new();
    store.write(page_id.clone(), final_dirty);
    let plan = RecoveryPlan::from_wal(&log);

    let report = plan.replay(&log, &mut store).expect("replay válido");

    assert_eq!(store.read(&page_id), Some(&original));
    assert_eq!(report.redone_records(), 0);
    assert_eq!(report.undone_records(), 2);
}

fn account_update(before: &str, after: &str) -> LogOperation {
    LogOperation::update(account_page(), page_image(before), page_image(after))
        .expect("update válido")
}

fn account_page() -> PageId {
    PageId::new("heap/accounts/0001").expect("page id válido")
}

fn page_image(value: &str) -> PageImage {
    PageImage::new(value).expect("imagen válida")
}
