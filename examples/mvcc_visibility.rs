use rust_database_internals::mvcc::{
    LogicalTimestamp, RecordId, RecordValue, RecordVersion, VersionId, VisibilityDecision,
};

fn main() {
    let record_id = RecordId::new("accounts/42").expect("id válido");
    let value = RecordValue::new("saldo=100").expect("valor válido");
    let mut version = RecordVersion::new(
        record_id,
        VersionId::new(1),
        LogicalTimestamp::new(10),
        value,
    );

    version
        .delete_at(LogicalTimestamp::new(12))
        .expect("cerrar la versión debe ser válido");

    assert_eq!(
        version.visibility_at(LogicalTimestamp::new(9)),
        VisibilityDecision::NotYetCreated {
            created_at: LogicalTimestamp::new(10),
            read_at: LogicalTimestamp::new(9),
        }
    );
    assert_eq!(
        version.visibility_at(LogicalTimestamp::new(11)),
        VisibilityDecision::Visible
    );
    assert_eq!(
        version.visibility_at(LogicalTimestamp::new(12)),
        VisibilityDecision::Deleted {
            deleted_at: LogicalTimestamp::new(12),
            read_at: LogicalTimestamp::new(12),
        }
    );

    println!("La ventana visible de v1 es [t10, t12).");
}
