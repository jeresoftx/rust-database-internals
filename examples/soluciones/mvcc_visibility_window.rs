use rust_database_internals::mvcc::{
    LogicalTimestamp, RecordId, RecordValue, RecordVersion, VersionId, VisibilityDecision,
};

fn main() {
    let mut version = RecordVersion::new(
        RecordId::new("accounts/42").expect("id válido"),
        VersionId::new(1),
        LogicalTimestamp::new(10),
        RecordValue::new("saldo=100").expect("valor válido"),
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
        version.visibility_at(LogicalTimestamp::new(10)),
        VisibilityDecision::Visible
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

    println!("Solución: la ventana visible de v1 es [t10, t12).");
}
