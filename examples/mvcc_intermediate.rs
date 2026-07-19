use rust_database_internals::mvcc::{
    LogicalTimestamp, RecordId, RecordValue, Snapshot, VersionChain, VersionId,
};

fn main() {
    let record_id = RecordId::new("accounts/42").expect("id válido");
    let mut chain = VersionChain::new(record_id);

    chain
        .append(
            LogicalTimestamp::new(10),
            RecordValue::new("saldo=100").expect("valor válido"),
        )
        .expect("la primera versión debe entrar");
    chain
        .delete_latest_at(LogicalTimestamp::new(12))
        .expect("cerrar v1 debe ser válido");
    chain
        .append(
            LogicalTimestamp::new(12),
            RecordValue::new("saldo=120").expect("valor válido"),
        )
        .expect("la segunda versión debe entrar");

    let old_snapshot = Snapshot::new(LogicalTimestamp::new(11));
    let new_snapshot = Snapshot::new(LogicalTimestamp::new(12));

    assert_eq!(
        chain
            .read(&old_snapshot)
            .expect("snapshot antiguo ve v1")
            .version_id(),
        VersionId::new(1)
    );
    assert_eq!(
        chain
            .read(&new_snapshot)
            .expect("snapshot nuevo ve v2")
            .version_id(),
        VersionId::new(2)
    );

    println!("Un lector antiguo ve v1; un lector nuevo ve v2.");
}
