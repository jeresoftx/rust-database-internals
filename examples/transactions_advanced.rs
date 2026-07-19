use rust_database_internals::transactions::{ResourceId, TransactionManager};

fn main() {
    let mut manager = TransactionManager::new();
    let first_attempt = manager
        .begin()
        .expect("abrir la primera transacción debe producir un id");
    let retry = manager
        .begin()
        .expect("abrir la transacción de reintento debe producir un id");
    let reservation = ResourceId::new("reservations/GGG74R").expect("recurso válido");

    manager
        .lock_exclusive(first_attempt, reservation.clone())
        .expect("la primera transacción toma la reserva");

    manager
        .rollback(first_attempt)
        .expect("rollback libera el recurso protegido");

    manager
        .lock_exclusive(retry, reservation.clone())
        .expect("el reintento puede tomar el recurso liberado");
    manager
        .commit(retry)
        .expect("commit cierra el reintento confirmado");

    assert_eq!(manager.lock_owner(&reservation), None);

    println!(
        "{} quedó libre después del commit del reintento.",
        reservation.as_str()
    );
}
