use rust_database_internals::wal::{
    LogOperation, PageId, PageImage, WalTransactionId, WriteAheadLog,
};

fn main() {
    let mut log = WriteAheadLog::new();
    let transaction_id = WalTransactionId::new(10);

    let begin_lsn = log.append_begin(transaction_id);
    let update = LogOperation::update(
        PageId::new("heap/accounts/0001").expect("page id válido"),
        PageImage::new("saldo=100").expect("imagen before válida"),
        PageImage::new("saldo=120").expect("imagen after válida"),
    )
    .expect("el cambio debe ser observable");
    let update_lsn = log.append(transaction_id, update);
    let commit_lsn = log.append_commit(transaction_id);

    assert_eq!(begin_lsn.value(), 1);
    assert_eq!(update_lsn.value(), 2);
    assert_eq!(commit_lsn.value(), 3);

    let operations: Vec<&str> = log.iter().map(|record| record.operation().name()).collect();
    assert_eq!(operations, vec!["begin", "update", "commit"]);

    println!("Solución: WAL preserva begin, update y commit en orden de LSN.");
}
