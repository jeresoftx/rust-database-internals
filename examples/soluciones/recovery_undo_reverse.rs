use rust_database_internals::{
    recovery::RecoveryPlan,
    wal::{LogOperation, PageId, PageImage, PageStore, WalTransactionId, WriteAheadLog},
};

fn main() {
    let page_id = PageId::new("heap/accounts/0001").expect("page id válido");
    let transaction_id = WalTransactionId::new(10);
    let mut log = WriteAheadLog::new();

    log.append_begin(transaction_id);
    log.append(transaction_id, account_update("saldo=100", "saldo=120"));
    log.append(transaction_id, account_update("saldo=120", "saldo=140"));

    let mut store = PageStore::new();
    store.write(
        page_id.clone(),
        PageImage::new("saldo=140").expect("imagen válida"),
    );

    let plan = RecoveryPlan::from_wal(&log);
    let report = plan.replay(&log, &mut store).expect("replay válido");

    assert_eq!(
        store.read(&page_id),
        Some(&PageImage::new("saldo=100").expect("imagen válida"))
    );
    assert_eq!(report.redone_records(), 0);
    assert_eq!(report.undone_records(), 2);

    println!("Solución: undo recorre el WAL hacia atrás y restaura saldo=100.");
}

fn account_update(before: &str, after: &str) -> LogOperation {
    LogOperation::update(
        PageId::new("heap/accounts/0001").expect("page id válido"),
        PageImage::new(before).expect("imagen before válida"),
        PageImage::new(after).expect("imagen after válida"),
    )
    .expect("update válido")
}
