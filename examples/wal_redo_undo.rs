use rust_database_internals::wal::{
    LogOperation, LogRecord, LogSequenceNumber, PageId, PageImage, PageStore, WalTransactionId,
};

fn main() {
    let page_id = PageId::new("heap/accounts/0001").expect("page id válido");
    let before = PageImage::new("saldo=100").expect("imagen válida");
    let after = PageImage::new("saldo=120").expect("imagen válida");
    let update = LogOperation::update(page_id.clone(), before.clone(), after.clone())
        .expect("update válido");
    let record = LogRecord::new(LogSequenceNumber::new(2), WalTransactionId::new(10), update);

    let mut store = PageStore::new();
    store.write(page_id.clone(), before.clone());

    store.redo(&record).expect("redo debe aplicar after");
    assert_eq!(store.read(&page_id), Some(&after));

    store.undo(&record).expect("undo debe restaurar before");
    assert_eq!(store.read(&page_id), Some(&before));

    println!("Redo aplicó after y undo restauró before para heap/accounts/0001.");
}
