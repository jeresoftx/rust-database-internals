use rust_database_internals::acid::{AcidError, OperationId, ReadCommittedCell};

fn main() {
    let mut balance = ReadCommittedCell::new("balance=100");
    let holder = OperationId::new(1);
    let requester = OperationId::new(2);

    balance
        .write_pending(holder, "balance=50")
        .expect("el primer escritor prepara el cambio");

    assert_eq!(balance.read_committed(), "balance=100");
    assert_eq!(
        balance.write_pending(requester, "balance=75"),
        Err(AcidError::PendingWriteConflict { holder, requester })
    );

    balance
        .rollback_pending(holder)
        .expect("rollback descarta la escritura pendiente");
    balance
        .write_pending(requester, "balance=75")
        .expect("el segundo escritor puede continuar después del rollback");
    balance
        .commit_pending(requester)
        .expect("el segundo escritor publica su cambio");

    assert_eq!(balance.read_committed(), "balance=75");

    println!("Ejercicio completado: la escritura pendiente no fue visible.");
}
