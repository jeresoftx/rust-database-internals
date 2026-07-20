use rust_database_internals::{
    replication::{ReplicationAckMode, ReplicationCluster, ReplicationDecision, ReplicationNode},
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

    println!("async confirmó con lag; sync confirmó después de replicar.");
}
