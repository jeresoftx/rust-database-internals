use rust_database_internals::acid::CommitLog;

fn main() {
    let mut log = CommitLog::new();
    let commit_id = log
        .append_commit("reservation-confirmed")
        .expect("el commit debe registrarse");

    let crash_before_sync = true;

    if crash_before_sync {
        assert_eq!(log.is_durable(commit_id), Some(false));
    }

    log.sync(commit_id)
        .expect("después de sync el commit se considera durable");

    assert_eq!(log.is_durable(commit_id), Some(true));
    assert_eq!(log.label(commit_id), Some("reservation-confirmed"));

    println!("Ejercicio completado: durable solo después de sync.");
}
