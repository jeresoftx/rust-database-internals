use rust_database_internals::mvcc::{
    LogicalTimestamp, RecordId, RecordValue, Snapshot, VersionChain,
};

fn main() {
    let mut chain = VersionChain::new(RecordId::new("accounts/42").expect("id válido"));

    chain
        .append(
            LogicalTimestamp::new(10),
            RecordValue::new("saldo=100").expect("valor válido"),
        )
        .expect("la versión inicial debe entrar");

    let snapshot = Snapshot::new(LogicalTimestamp::new(10));
    let visible = chain
        .read(&snapshot)
        .expect("el snapshot debe ver la versión");

    assert_eq!(visible.value().as_str(), "saldo=100");
    assert_eq!(visible.version_id().value(), 1);

    println!("Solución: el snapshot t10 lee v1 con saldo=100.");
}
