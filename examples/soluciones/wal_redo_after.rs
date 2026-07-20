use rust_database_internals::wal::{
    LogOperation, LogRecord, LogSequenceNumber, PageId, PageImage, PageStore, WalTransactionId,
};

fn main() {
    let page_id = PageId::new("heap/accounts/0001").expect("page id válido");
    let before = PageImage::new("saldo=100").expect("imagen before válida");
    let after = PageImage::new("saldo=120").expect("imagen after válida");
    let update = LogOperation::update(page_id.clone(), before.clone(), after.clone())
        .expect("el cambio debe ser observable");
    let record = LogRecord::new(LogSequenceNumber::new(2), WalTransactionId::new(10), update);

    let mut store = PageStore::new();
    store.write(page_id.clone(), before);

    store.redo(&record).expect("redo debe escribir after");

    assert_eq!(store.read(&page_id), Some(&after));

    println!("Solución: redo aplica la imagen after del registro update.");
}
