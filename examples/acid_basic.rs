use rust_database_internals::acid::{AtomicUnit, Change, UnitState};

fn main() {
    let mut unit = AtomicUnit::new();

    unit.stage(Change::from("debit account"))
        .expect("el débito debe prepararse");
    unit.stage(Change::from("credit account"))
        .expect("el crédito debe prepararse");
    unit.commit().expect("commit debe cerrar la unidad");

    assert_eq!(unit.state(), UnitState::Committed);
    assert!(unit.staged_changes().is_empty());
    assert_eq!(unit.committed_changes().len(), 2);

    println!("Atomicity: dos cambios se confirmaron como una sola unidad.");
}
