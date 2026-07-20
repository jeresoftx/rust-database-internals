use rust_database_internals::wal::{
    LogOperation, PageId, PageImage, PageStore, WalTransactionId, WriteAheadLog,
};

fn main() {
    let page_id = PageId::new("heap/accounts/0001").expect("page id válido");
    let before = PageImage::new("saldo=100").expect("imagen before válida");
    let after = PageImage::new("saldo=120").expect("imagen after válida");
    let transaction_id = WalTransactionId::new(10);

    let mut log = WriteAheadLog::new();
    let mut store = PageStore::new();
    store.write(page_id.clone(), before.clone());

    log.append_begin(transaction_id);
    let update = LogOperation::update(page_id.clone(), before, after.clone())
        .expect("el cambio debe ser observable");
    let update_lsn = log.append(transaction_id, update);

    let record = log.records().last().expect("el update ya vive en WAL");
    store
        .redo(record)
        .expect("la página se modifica después del WAL");

    assert_eq!(update_lsn.value(), 2);
    assert_eq!(store.read(&page_id), Some(&after));

    println!("Regla WAL: LSN 2 se escribió antes de modificar heap/accounts/0001.");
}
