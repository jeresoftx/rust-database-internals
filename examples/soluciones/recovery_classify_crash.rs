use rust_database_internals::{
    recovery::RecoveryPlan,
    wal::{LogOperation, PageId, PageImage, WalTransactionId, WriteAheadLog},
};

fn main() {
    let transaction_id = WalTransactionId::new(10);
    let mut log = WriteAheadLog::new();

    log.append_begin(transaction_id);
    log.append(transaction_id, account_update("saldo=100", "saldo=120"));

    let before_commit = RecoveryPlan::from_wal(&log);
    assert!(before_commit.requires_undo(transaction_id));
    assert!(!before_commit.requires_redo(transaction_id));

    log.append_commit(transaction_id);

    let after_commit = RecoveryPlan::from_wal(&log);
    assert!(after_commit.requires_redo(transaction_id));
    assert!(!after_commit.requires_undo(transaction_id));

    println!("Solución: antes de commit requiere undo; después de commit requiere redo.");
}

fn account_update(before: &str, after: &str) -> LogOperation {
    LogOperation::update(
        PageId::new("heap/accounts/0001").expect("page id válido"),
        PageImage::new(before).expect("imagen before válida"),
        PageImage::new(after).expect("imagen after válida"),
    )
    .expect("update válido")
}
