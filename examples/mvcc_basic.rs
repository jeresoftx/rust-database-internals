use rust_database_internals::mvcc::{
    LogicalTimestamp, RecordId, RecordValue, Snapshot, VersionChain,
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

    let snapshot = Snapshot::new(LogicalTimestamp::new(10));
    let visible = chain.read(&snapshot).expect("el snapshot debe ver v1");

    assert_eq!(visible.value().as_str(), "saldo=100");

    println!(
        "Snapshot t10 lee la versión {}.",
        visible.version_id().value()
    );
}
