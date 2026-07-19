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
    chain
        .delete_latest_at(LogicalTimestamp::new(12))
        .expect("cerrar v1 debe ser válido");

    let before_delete = Snapshot::new(LogicalTimestamp::new(11));
    let after_delete = Snapshot::new(LogicalTimestamp::new(12));

    assert_eq!(
        chain
            .read(&before_delete)
            .expect("antes del cierre la versión sigue visible")
            .value()
            .as_str(),
        "saldo=100"
    );
    assert_eq!(chain.read(&after_delete), None);

    println!("Después del borrado lógico, un snapshot nuevo ya no ve el registro.");
}
