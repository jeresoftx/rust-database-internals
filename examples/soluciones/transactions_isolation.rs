use rust_database_internals::transactions::{ResourceId, TransactionManager};

fn main() {
    let mut manager = TransactionManager::new();
    let holder = manager
        .begin()
        .expect("abrir la primera transacción debe producir un id");
    let next = manager
        .begin()
        .expect("abrir la segunda transacción debe producir un id");
    let resource = ResourceId::new("orders/1001").expect("recurso válido");

    manager
        .lock_exclusive(holder, resource.clone())
        .expect("la primera transacción toma el recurso");
    manager
        .rollback(holder)
        .expect("rollback libera los locks de la transacción");

    manager
        .lock_exclusive(next, resource.clone())
        .expect("otra transacción puede tomar el recurso liberado");

    assert_eq!(manager.lock_owner(&resource), Some(next));

    println!("Ejercicio completado: cerrar una transacción libera su lock.");
}
