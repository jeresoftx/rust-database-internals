use rust_database_internals::{
    replication::{ReplicationCluster, ReplicationNode},
    wal::{LogOperation, PageId, PageImage, WalTransactionId},
};

fn main() {
    let mut primary = ReplicationNode::primary("primary-1").expect("primary válido");
    let transaction_id = WalTransactionId::new(10);

    primary
        .append_local_update(transaction_id, account_update("saldo=100", "saldo=120"))
        .expect("primary debe aceptar escritura");
    primary
        .append_local_commit(transaction_id)
        .expect("commit local válido");

    let replica = ReplicationNode::replica("replica-1").expect("réplica válida");
    let mut cluster = ReplicationCluster::new(primary, vec![replica]).expect("cluster válido");

    let report = cluster
        .replicate_to("replica-1")
        .expect("replicación válida");

    assert_eq!(report.copied_records(), 2);
    assert_eq!(
        cluster
            .replica("replica-1")
            .expect("réplica")
            .log()
            .last_lsn(),
        cluster.primary().log().last_lsn()
    );

    println!("Solución: la réplica copió los 2 registros del WAL del primary.");
}

fn account_update(before: &str, after: &str) -> LogOperation {
    LogOperation::update(
        PageId::new("heap/accounts/0001").expect("page id válido"),
        PageImage::new(before).expect("imagen before válida"),
        PageImage::new(after).expect("imagen after válida"),
    )
    .expect("update válido")
}
