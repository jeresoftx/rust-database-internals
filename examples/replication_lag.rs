use rust_database_internals::{
    replication::{ReplicationCluster, ReplicationNode},
    wal::{LogOperation, PageId, PageImage, WalTransactionId},
};

fn main() {
    let mut primary = ReplicationNode::primary("primary-1").expect("primary válido");
    let transaction_id = WalTransactionId::new(10);

    primary
        .append_local_update(
            transaction_id,
            LogOperation::update(
                PageId::new("heap/accounts/0001").expect("page id válido"),
                PageImage::new("saldo=100").expect("imagen before válida"),
                PageImage::new("saldo=120").expect("imagen after válida"),
            )
            .expect("update válido"),
        )
        .expect("primary acepta escritura");
    primary
        .append_local_commit(transaction_id)
        .expect("commit local válido");

    let replica = ReplicationNode::replica("replica-1").expect("réplica válida");
    let mut cluster = ReplicationCluster::new(primary, vec![replica]).expect("cluster válido");

    let before = cluster
        .replica_lag("replica-1")
        .expect("lag de réplica conocida");
    assert_eq!(before.pending_records(), 2);
    assert!(!before.is_caught_up());

    cluster
        .replicate_to("replica-1")
        .expect("replicación válida");

    let after = cluster
        .replica_lag("replica-1")
        .expect("lag de réplica conocida");
    assert_eq!(after.pending_records(), 0);
    assert!(after.is_caught_up());

    println!("replica-1 pasó de 2 registros pendientes a 0.");
}
