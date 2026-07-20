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
    chain
        .delete_latest_at(LogicalTimestamp::new(12))
        .expect("cerrar la versión debe ser válido");

    let old_snapshot = Snapshot::new(LogicalTimestamp::new(11));
    let new_snapshot = Snapshot::new(LogicalTimestamp::new(12));

    assert_eq!(
        chain
            .read(&old_snapshot)
            .expect("el snapshot anterior al cierre todavía ve la versión")
            .value()
            .as_str(),
        "saldo=100"
    );
    assert!(chain.read(&new_snapshot).is_none());

    println!("Solución: el borrado lógico conserva historia para lectores antiguos.");
}
