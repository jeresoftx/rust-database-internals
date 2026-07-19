use rust_database_internals::acid::{AtomicUnit, Change, UnitState};

fn main() {
    let mut transfer = AtomicUnit::new();

    transfer
        .stage(Change::from("debit account A"))
        .expect("el débito debe prepararse");
    transfer
        .stage(Change::from("credit account B"))
        .expect("el crédito debe prepararse");

    let network_error_before_commit = true;

    if network_error_before_commit {
        transfer
            .rollback()
            .expect("rollback debe descartar todos los cambios tentativos");
    }

    assert_eq!(transfer.state(), UnitState::RolledBack);
    assert!(transfer.staged_changes().is_empty());
    assert!(transfer.committed_changes().is_empty());

    println!("Ejercicio completado: la falla parcial terminó en rollback total.");
}
