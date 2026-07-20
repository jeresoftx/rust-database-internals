use rust_database_internals::{
    recovery::RecoveryPlan,
    wal::{LogOperation, PageId, PageImage, PageStore, WalTransactionId, WriteAheadLog},
};

fn main() {
    let page_id = PageId::new("heap/accounts/0001").expect("page id válido");
    let mut log = WriteAheadLog::new();
    let committed = WalTransactionId::new(10);
    let uncommitted = WalTransactionId::new(20);

    log.append_begin(committed);
    log.append(committed, account_update("saldo=100", "saldo=120"));
    log.append_commit(committed);

    log.append_begin(uncommitted);
    log.append(uncommitted, account_update("saldo=120", "saldo=140"));

    let mut store = PageStore::new();
    store.write(
        page_id.clone(),
        PageImage::new("saldo=140").expect("imagen válida"),
    );

    let plan = RecoveryPlan::from_wal(&log);
    let report = plan.replay(&log, &mut store).expect("replay válido");

    assert_eq!(
        store.read(&page_id),
        Some(&PageImage::new("saldo=120").expect("imagen válida"))
    );
    assert_eq!(report.redone_records(), 1);
    assert_eq!(report.undone_records(), 1);

    println!("Replay aplicó 1 redo y 1 undo; saldo final recuperado: saldo=120.");
}

fn account_update(before: &str, after: &str) -> LogOperation {
    LogOperation::update(
        PageId::new("heap/accounts/0001").expect("page id válido"),
        PageImage::new(before).expect("imagen before válida"),
        PageImage::new(after).expect("imagen after válida"),
    )
    .expect("update válido")
}
