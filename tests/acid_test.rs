use rust_database_internals::acid::{
    AcidError, AcidProperty, AtomicUnit, Change, CommitId, CommitLog, OperationId,
    ReadCommittedCell, UniqueConstraint, UnitState,
};

#[test]
fn acid_property_names_are_stable() {
    assert_eq!(AcidProperty::Atomicity.as_str(), "atomicity");
    assert_eq!(AcidProperty::Consistency.as_str(), "consistency");
    assert_eq!(AcidProperty::Isolation.as_str(), "isolation");
    assert_eq!(AcidProperty::Durability.as_str(), "durability");
}

#[test]
fn atomic_unit_commits_all_staged_changes() {
    let mut unit = AtomicUnit::new();

    unit.stage(Change::from("debit account"))
        .expect("un cambio debe poder prepararse");
    unit.stage(Change::from("credit account"))
        .expect("un cambio debe poder prepararse");
    unit.commit().expect("commit debe cerrar la unidad");

    assert_eq!(unit.state(), UnitState::Committed);
    assert!(unit.staged_changes().is_empty());
    assert_eq!(
        unit.committed_changes(),
        &[
            Change::from("debit account"),
            Change::from("credit account")
        ]
    );
}

#[test]
fn atomic_unit_rollback_discards_all_staged_changes() {
    let mut unit = AtomicUnit::new();

    unit.stage(Change::from("reserve seat"))
        .expect("un cambio debe poder prepararse");
    unit.rollback().expect("rollback debe cerrar la unidad");

    assert_eq!(unit.state(), UnitState::RolledBack);
    assert!(unit.staged_changes().is_empty());
    assert!(unit.committed_changes().is_empty());
}

#[test]
fn atomic_unit_rejects_changes_after_commit() {
    let mut unit = AtomicUnit::new();

    unit.commit().expect("commit debe cerrar la unidad");

    assert_eq!(
        unit.stage(Change::from("late write")),
        Err(AcidError::ClosedAtomicUnit {
            state: UnitState::Committed,
        })
    );
}

#[test]
fn unique_constraint_accepts_unique_values() {
    let mut constraint =
        UniqueConstraint::new("customers.email").expect("el nombre debe ser válido");

    constraint
        .insert("ana@example.com")
        .expect("el primer valor debe aceptarse");
    constraint
        .insert("luis@example.com")
        .expect("un valor distinto debe aceptarse");

    assert_eq!(constraint.len(), 2);
    assert!(constraint.contains("ana@example.com"));
}

#[test]
fn unique_constraint_rejects_duplicate_values() {
    let mut constraint =
        UniqueConstraint::new("customers.email").expect("el nombre debe ser válido");

    constraint
        .insert("ana@example.com")
        .expect("el primer valor debe aceptarse");

    assert_eq!(
        constraint.insert("ana@example.com"),
        Err(AcidError::ConsistencyViolation {
            invariant: "customers.email".to_owned(),
            value: "ana@example.com".to_owned(),
        })
    );
}

#[test]
fn read_committed_model_hides_pending_write_until_commit() {
    let mut cell = ReadCommittedCell::new("balance=100");
    let writer = OperationId::new(1);

    cell.write_pending(writer, "balance=50")
        .expect("el escritor debe poder preparar el cambio");

    assert_eq!(cell.read_committed(), "balance=100");
    assert_eq!(cell.pending_writer(), Some(writer));

    cell.commit_pending(writer)
        .expect("el escritor pendiente debe poder confirmar");

    assert_eq!(cell.read_committed(), "balance=50");
    assert_eq!(cell.pending_writer(), None);
}

#[test]
fn read_committed_model_detects_conflicting_pending_writer() {
    let mut cell = ReadCommittedCell::new("balance=100");
    let holder = OperationId::new(1);
    let requester = OperationId::new(2);

    cell.write_pending(holder, "balance=50")
        .expect("el primer escritor debe poder preparar el cambio");

    assert_eq!(
        cell.write_pending(requester, "balance=75"),
        Err(AcidError::PendingWriteConflict { holder, requester })
    );
    assert_eq!(cell.read_committed(), "balance=100");
}

#[test]
fn durability_log_marks_commit_durable_only_after_sync() {
    let mut log = CommitLog::new();

    let commit_id = log
        .append_commit("transaction-1")
        .expect("un commit debe poder agregarse al log");

    assert_eq!(commit_id, CommitId::new(1));
    assert_eq!(log.is_durable(commit_id), Some(false));

    log.sync(commit_id)
        .expect("sync debe marcar durable un commit conocido");

    assert_eq!(log.is_durable(commit_id), Some(true));
}

#[test]
fn durability_log_rejects_unknown_commit_record() {
    let mut log = CommitLog::new();
    let unknown = CommitId::new(404);

    assert_eq!(log.sync(unknown), Err(AcidError::UnknownCommit(unknown)));
    assert_eq!(log.is_durable(unknown), None);
}
