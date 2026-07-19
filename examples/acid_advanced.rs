use rust_database_internals::acid::{CommitLog, OperationId, ReadCommittedCell};

fn main() {
    let mut balance = ReadCommittedCell::new("balance=100");
    let writer = OperationId::new(1);

    balance
        .write_pending(writer, "balance=50")
        .expect("el escritor debe preparar el cambio");

    assert_eq!(balance.read_committed(), "balance=100");

    balance
        .commit_pending(writer)
        .expect("el escritor puede publicar el cambio");

    assert_eq!(balance.read_committed(), "balance=50");

    let mut log = CommitLog::new();
    let commit_id = log
        .append_commit("transfer-1001")
        .expect("el commit debe registrarse");

    assert_eq!(log.is_durable(commit_id), Some(false));
    log.sync(commit_id)
        .expect("sync marca el commit como durable");
    assert_eq!(log.is_durable(commit_id), Some(true));

    println!("Isolation y Durability: el cambio se publicó y luego se sincronizó.");
}
