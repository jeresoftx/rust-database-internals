use rust_database_internals::{
    replication::{ReplicationAckMode, ReplicationCluster, ReplicationDecision, ReplicationNode},
    wal::{LogOperation, PageId, PageImage, WalTransactionId},
};

fn main() {
    let mut cluster = cluster_with_lagged_replica();

    assert_eq!(
        cluster
            .confirm_write(ReplicationAckMode::Async)
            .expect("confirmación válida"),
        ReplicationDecision::Confirmed
    );
    assert_eq!(
        cluster
            .confirm_write(ReplicationAckMode::Sync)
            .expect("confirmación válida"),
        ReplicationDecision::WaitingForReplicas {
            pending_replicas: 1,
            pending_records: 2,
        }
    );

    cluster
        .replicate_to("replica-1")
        .expect("replicación válida");

    assert_eq!(
        cluster
            .confirm_write(ReplicationAckMode::Sync)
            .expect("confirmación válida"),
        ReplicationDecision::Confirmed
    );

    println!("Solución: async confirma rápido; sync espera a que la réplica alcance al primary.");
}

fn cluster_with_lagged_replica() -> ReplicationCluster {
    let mut primary = ReplicationNode::primary("primary-1").expect("primary válido");
    let transaction_id = WalTransactionId::new(10);

    primary
        .append_local_update(transaction_id, account_update("saldo=100", "saldo=120"))
        .expect("primary debe aceptar escritura");
    primary
        .append_local_commit(transaction_id)
        .expect("commit local válido");

    let replica = ReplicationNode::replica("replica-1").expect("réplica válida");
    ReplicationCluster::new(primary, vec![replica]).expect("cluster válido")
}

fn account_update(before: &str, after: &str) -> LogOperation {
    LogOperation::update(
        PageId::new("heap/accounts/0001").expect("page id válido"),
        PageImage::new(before).expect("imagen before válida"),
        PageImage::new(after).expect("imagen after válida"),
    )
    .expect("update válido")
}
