use rust_database_internals::mvcc::{
    LogicalTimestamp, MvccError, RecordId, RecordValue, RecordVersion, Snapshot, VersionChain,
    VersionId, VisibilityDecision,
};

#[test]
fn record_id_rejects_blank_text() {
    assert_eq!(RecordId::new("   "), Err(MvccError::BlankRecordId));
}

#[test]
fn record_id_trims_and_exposes_stable_name() {
    let record_id = RecordId::new(" accounts/42 ").expect("id de registro válido");

    assert_eq!(record_id.as_str(), "accounts/42");
}

#[test]
fn record_value_rejects_blank_text() {
    assert_eq!(RecordValue::new("   "), Err(MvccError::BlankRecordValue));
}

#[test]
fn record_value_trims_and_exposes_payload() {
    let value = RecordValue::new(" saldo=100 ").expect("valor de registro válido");

    assert_eq!(value.as_str(), "saldo=100");
}

#[test]
fn logical_timestamp_exposes_value() {
    let timestamp = LogicalTimestamp::new(42);

    assert_eq!(timestamp.value(), 42);
}

#[test]
fn snapshot_exposes_read_timestamp() {
    let snapshot = Snapshot::new(LogicalTimestamp::new(42));

    assert_eq!(snapshot.read_at(), LogicalTimestamp::new(42));
}

#[test]
fn version_id_exposes_value() {
    let version_id = VersionId::new(7);

    assert_eq!(version_id.value(), 7);
}

#[test]
fn record_version_exposes_visibility_metadata() {
    let record_id = RecordId::new("accounts/42").expect("id válido");
    let value = RecordValue::new("saldo=100").expect("valor válido");

    let version = RecordVersion::new(
        record_id.clone(),
        VersionId::new(1),
        LogicalTimestamp::new(10),
        value.clone(),
    );

    assert_eq!(version.record_id(), &record_id);
    assert_eq!(version.version_id(), VersionId::new(1));
    assert_eq!(version.created_at(), LogicalTimestamp::new(10));
    assert_eq!(version.deleted_at(), None);
    assert_eq!(version.value(), &value);
    assert!(!version.is_deleted());
}

#[test]
fn record_version_can_be_marked_deleted() {
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
        .expect("borrado lógico después de creación debe ser válido");

    assert_eq!(version.deleted_at(), Some(LogicalTimestamp::new(12)));
    assert!(version.is_deleted());
}

#[test]
fn record_version_rejects_delete_before_create() {
    let record_id = RecordId::new("accounts/42").expect("id válido");
    let value = RecordValue::new("saldo=100").expect("valor válido");
    let mut version = RecordVersion::new(
        record_id,
        VersionId::new(1),
        LogicalTimestamp::new(10),
        value,
    );

    assert_eq!(
        version.delete_at(LogicalTimestamp::new(9)),
        Err(MvccError::DeleteBeforeCreate {
            created_at: LogicalTimestamp::new(10),
            deleted_at: LogicalTimestamp::new(9),
        })
    );
}

#[test]
fn record_version_rejects_second_delete() {
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
        .expect("primer borrado lógico debe entrar");

    assert_eq!(
        version.delete_at(LogicalTimestamp::new(13)),
        Err(MvccError::VersionAlreadyDeleted {
            version_id: VersionId::new(1),
        })
    );
}

#[test]
fn record_version_is_visible_after_creation() {
    let record_id = RecordId::new("accounts/42").expect("id válido");
    let value = RecordValue::new("saldo=100").expect("valor válido");
    let version = RecordVersion::new(
        record_id,
        VersionId::new(1),
        LogicalTimestamp::new(10),
        value,
    );
    let snapshot = Snapshot::new(LogicalTimestamp::new(10));

    assert!(version.is_visible_in(&snapshot));
}

#[test]
fn visibility_at_reports_not_yet_created_before_creation_boundary() {
    let record_id = RecordId::new("accounts/42").expect("id válido");
    let value = RecordValue::new("saldo=100").expect("valor válido");
    let version = RecordVersion::new(
        record_id,
        VersionId::new(1),
        LogicalTimestamp::new(10),
        value,
    );

    assert_eq!(
        version.visibility_at(LogicalTimestamp::new(9)),
        VisibilityDecision::NotYetCreated {
            created_at: LogicalTimestamp::new(10),
            read_at: LogicalTimestamp::new(9),
        }
    );
}

#[test]
fn visibility_at_reports_visible_at_creation_boundary() {
    let record_id = RecordId::new("accounts/42").expect("id válido");
    let value = RecordValue::new("saldo=100").expect("valor válido");
    let version = RecordVersion::new(
        record_id,
        VersionId::new(1),
        LogicalTimestamp::new(10),
        value,
    );

    assert_eq!(
        version.visibility_at(LogicalTimestamp::new(10)),
        VisibilityDecision::Visible
    );
    assert!(version.is_visible_at(LogicalTimestamp::new(10)));
}

#[test]
fn record_version_is_not_visible_before_creation() {
    let record_id = RecordId::new("accounts/42").expect("id válido");
    let value = RecordValue::new("saldo=100").expect("valor válido");
    let version = RecordVersion::new(
        record_id,
        VersionId::new(1),
        LogicalTimestamp::new(10),
        value,
    );
    let snapshot = Snapshot::new(LogicalTimestamp::new(9));

    assert!(!version.is_visible_in(&snapshot));
}

#[test]
fn record_version_is_not_visible_at_delete_timestamp() {
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
        .expect("borrado lógico válido");
    let snapshot = Snapshot::new(LogicalTimestamp::new(12));

    assert!(!version.is_visible_in(&snapshot));
}

#[test]
fn visibility_at_reports_visible_just_before_delete_boundary() {
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
        .expect("borrado lógico válido");

    assert_eq!(
        version.visibility_at(LogicalTimestamp::new(11)),
        VisibilityDecision::Visible
    );
}

#[test]
fn visibility_at_reports_deleted_at_delete_boundary() {
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
        .expect("borrado lógico válido");

    assert_eq!(
        version.visibility_at(LogicalTimestamp::new(12)),
        VisibilityDecision::Deleted {
            deleted_at: LogicalTimestamp::new(12),
            read_at: LogicalTimestamp::new(12),
        }
    );
    assert!(!version.is_visible_at(LogicalTimestamp::new(12)));
}

#[test]
fn version_chain_starts_empty_for_record() {
    let record_id = RecordId::new("accounts/42").expect("id válido");
    let chain = VersionChain::new(record_id.clone());

    assert_eq!(chain.record_id(), &record_id);
    assert!(chain.is_empty());
    assert_eq!(chain.len(), 0);
    assert_eq!(chain.latest(), None);
}

#[test]
fn version_chain_reads_none_when_empty() {
    let record_id = RecordId::new("accounts/42").expect("id válido");
    let chain = VersionChain::new(record_id);
    let snapshot = Snapshot::new(LogicalTimestamp::new(10));

    assert_eq!(chain.read(&snapshot), None);
}

#[test]
fn version_chain_read_at_matches_snapshot_read() {
    let record_id = RecordId::new("accounts/42").expect("id válido");
    let mut chain = VersionChain::new(record_id);
    chain
        .append(
            LogicalTimestamp::new(10),
            RecordValue::new("saldo=100").expect("valor válido"),
        )
        .expect("primera versión debe entrar");
    let read_at = LogicalTimestamp::new(10);
    let snapshot = Snapshot::new(read_at);

    assert_eq!(chain.read_at(read_at), chain.read(&snapshot));
}

#[test]
fn version_chain_reads_none_before_first_version() {
    let record_id = RecordId::new("accounts/42").expect("id válido");
    let mut chain = VersionChain::new(record_id);
    chain
        .append(
            LogicalTimestamp::new(10),
            RecordValue::new("saldo=100").expect("valor válido"),
        )
        .expect("primera versión debe entrar");
    let snapshot = Snapshot::new(LogicalTimestamp::new(9));

    assert_eq!(chain.read(&snapshot), None);
}

#[test]
fn version_chain_reads_original_version_for_old_snapshot() {
    let record_id = RecordId::new("accounts/42").expect("id válido");
    let mut chain = VersionChain::new(record_id);
    chain
        .append(
            LogicalTimestamp::new(10),
            RecordValue::new("saldo=100").expect("valor válido"),
        )
        .expect("primera versión debe entrar");
    chain
        .delete_latest_at(LogicalTimestamp::new(12))
        .expect("cerrar la versión anterior debe ser válido");
    chain
        .append(
            LogicalTimestamp::new(12),
            RecordValue::new("saldo=120").expect("valor válido"),
        )
        .expect("segunda versión debe entrar");
    let snapshot = Snapshot::new(LogicalTimestamp::new(11));

    let visible = chain.read(&snapshot).expect("snapshot antiguo ve v1");

    assert_eq!(visible.version_id(), VersionId::new(1));
    assert_eq!(visible.value().as_str(), "saldo=100");
}

#[test]
fn version_chain_reads_newer_version_for_later_snapshot() {
    let record_id = RecordId::new("accounts/42").expect("id válido");
    let mut chain = VersionChain::new(record_id);
    chain
        .append(
            LogicalTimestamp::new(10),
            RecordValue::new("saldo=100").expect("valor válido"),
        )
        .expect("primera versión debe entrar");
    chain
        .delete_latest_at(LogicalTimestamp::new(12))
        .expect("cerrar la versión anterior debe ser válido");
    chain
        .append(
            LogicalTimestamp::new(12),
            RecordValue::new("saldo=120").expect("valor válido"),
        )
        .expect("segunda versión debe entrar");
    let snapshot = Snapshot::new(LogicalTimestamp::new(12));

    let visible = chain.read(&snapshot).expect("snapshot nuevo ve v2");

    assert_eq!(visible.version_id(), VersionId::new(2));
    assert_eq!(visible.value().as_str(), "saldo=120");
}

#[test]
fn version_chain_reads_none_after_delete_without_replacement() {
    let record_id = RecordId::new("accounts/42").expect("id válido");
    let mut chain = VersionChain::new(record_id);
    chain
        .append(
            LogicalTimestamp::new(10),
            RecordValue::new("saldo=100").expect("valor válido"),
        )
        .expect("primera versión debe entrar");
    chain
        .delete_latest_at(LogicalTimestamp::new(12))
        .expect("cerrar la versión debe ser válido");
    let snapshot = Snapshot::new(LogicalTimestamp::new(12));

    assert_eq!(chain.read(&snapshot), None);
}

#[test]
fn version_chain_rejects_delete_when_empty() {
    let record_id = RecordId::new("accounts/42").expect("id válido");
    let mut chain = VersionChain::new(record_id);

    assert_eq!(
        chain.delete_latest_at(LogicalTimestamp::new(12)),
        Err(MvccError::EmptyVersionChain)
    );
}

#[test]
fn version_chain_appends_versions_with_sequential_ids() {
    let record_id = RecordId::new("accounts/42").expect("id válido");
    let mut chain = VersionChain::new(record_id.clone());

    let first = chain
        .append(
            LogicalTimestamp::new(10),
            RecordValue::new("saldo=100").expect("valor válido"),
        )
        .expect("primera versión debe entrar");
    let second = chain
        .append(
            LogicalTimestamp::new(11),
            RecordValue::new("saldo=120").expect("valor válido"),
        )
        .expect("segunda versión debe entrar");

    assert_eq!(first, VersionId::new(1));
    assert_eq!(second, VersionId::new(2));
    assert_eq!(chain.len(), 2);
    assert_eq!(chain.versions()[0].record_id(), &record_id);
    assert_eq!(chain.versions()[0].value().as_str(), "saldo=100");
    assert_eq!(chain.versions()[1].value().as_str(), "saldo=120");
}

#[test]
fn version_chain_keeps_latest_version() {
    let record_id = RecordId::new("accounts/42").expect("id válido");
    let mut chain = VersionChain::new(record_id);
    chain
        .append(
            LogicalTimestamp::new(10),
            RecordValue::new("saldo=100").expect("valor válido"),
        )
        .expect("primera versión debe entrar");
    chain
        .append(
            LogicalTimestamp::new(11),
            RecordValue::new("saldo=120").expect("valor válido"),
        )
        .expect("segunda versión debe entrar");

    let latest = chain.latest().expect("debe existir versión más reciente");

    assert_eq!(latest.version_id(), VersionId::new(2));
    assert_eq!(latest.value().as_str(), "saldo=120");
}

#[test]
fn version_chain_rejects_non_monotonic_created_timestamp() {
    let record_id = RecordId::new("accounts/42").expect("id válido");
    let mut chain = VersionChain::new(record_id);
    chain
        .append(
            LogicalTimestamp::new(10),
            RecordValue::new("saldo=100").expect("valor válido"),
        )
        .expect("primera versión debe entrar");

    assert_eq!(
        chain.append(
            LogicalTimestamp::new(9),
            RecordValue::new("saldo=120").expect("valor válido"),
        ),
        Err(MvccError::NonMonotonicTimestamp {
            previous: LogicalTimestamp::new(10),
            next: LogicalTimestamp::new(9),
        })
    );
}
