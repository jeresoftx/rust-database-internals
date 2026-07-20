use rust_database_internals::{
    replication::{ReplicationCluster, ReplicationError, ReplicationNode, ReplicationRole},
    wal::{LogOperation, PageId, PageImage, WalTransactionId},
};

#[test]
fn primary_node_accepts_local_writes() {
    let mut primary = ReplicationNode::primary("primary-1").expect("primary válido");
    let transaction_id = WalTransactionId::new(10);

    primary
        .append_local_update(transaction_id, account_update("saldo=100", "saldo=120"))
        .expect("primary debe aceptar escrituras locales");

    assert_eq!(primary.role(), ReplicationRole::Primary);
    assert_eq!(primary.log().len(), 1);
}

#[test]
fn replica_node_rejects_local_writes() {
    let mut replica = ReplicationNode::replica("replica-1").expect("réplica válida");
    let transaction_id = WalTransactionId::new(10);

    let result =
        replica.append_local_update(transaction_id, account_update("saldo=100", "saldo=120"));

    assert_eq!(
        result,
        Err(ReplicationError::ReplicaCannotAcceptLocalWrite {
            node_id: "replica-1".to_owned(),
        })
    );
    assert_eq!(replica.log().len(), 0);
}

#[test]
fn cluster_requires_exactly_one_primary() {
    let primary = ReplicationNode::primary("primary-1").expect("primary válido");
    let replica = ReplicationNode::replica("replica-1").expect("réplica válida");

    let cluster = ReplicationCluster::new(primary, vec![replica]).expect("cluster válido");

    assert_eq!(cluster.primary().id(), "primary-1");
    assert_eq!(cluster.replicas().len(), 1);
}

#[test]
fn cluster_replicates_primary_log_to_replica() {
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
        cluster.replica("replica-1").expect("réplica").log().len(),
        2
    );
    assert_eq!(
        cluster
            .replica("replica-1")
            .expect("réplica")
            .log()
            .last_lsn(),
        cluster.primary().log().last_lsn()
    );
}

#[test]
fn cluster_reports_replica_lag_before_and_after_replication() {
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

    let lag = cluster
        .replica_lag("replica-1")
        .expect("lag de réplica conocida");

    assert_eq!(lag.pending_records(), 2);
    assert_eq!(lag.primary_records(), 2);
    assert_eq!(lag.replica_records(), 0);
    assert!(!lag.is_caught_up());

    cluster
        .replicate_to("replica-1")
        .expect("replicación válida");
    let caught_up = cluster
        .replica_lag("replica-1")
        .expect("lag de réplica conocida");

    assert_eq!(caught_up.pending_records(), 0);
    assert_eq!(
        caught_up.primary_last_lsn(),
        cluster.primary().log().last_lsn()
    );
    assert_eq!(
        caught_up.replica_last_lsn(),
        cluster
            .replica("replica-1")
            .expect("réplica")
            .log()
            .last_lsn()
    );
    assert!(caught_up.is_caught_up());
}

#[test]
fn cluster_rejects_lag_for_unknown_replica() {
    let primary = ReplicationNode::primary("primary-1").expect("primary válido");
    let cluster = ReplicationCluster::new(primary, vec![]).expect("cluster válido");

    assert_eq!(
        cluster.replica_lag("replica-ausente"),
        Err(ReplicationError::UnknownReplica {
            node_id: "replica-ausente".to_owned(),
        })
    );
}

fn account_update(before: &str, after: &str) -> LogOperation {
    LogOperation::update(
        PageId::new("heap/accounts/0001").expect("page id válido"),
        PageImage::new(before).expect("imagen before válida"),
        PageImage::new(after).expect("imagen after válida"),
    )
    .expect("update válido")
}
