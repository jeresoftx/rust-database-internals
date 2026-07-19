use rust_database_internals::transactions::{ResourceId, TransactionError, TransactionManager};

fn main() {
    let mut manager = TransactionManager::new();
    let holder = manager
        .begin()
        .expect("abrir la primera transacción debe producir un id");
    let requester = manager
        .begin()
        .expect("abrir la segunda transacción debe producir un id");
    let resource = ResourceId::new("accounts/42").expect("recurso válido");

    manager
        .lock_exclusive(holder, resource.clone())
        .expect("la primera transacción toma el lock");

    assert_eq!(
        manager.lock_exclusive(requester, resource.clone()),
        Err(TransactionError::ResourceConflict {
            resource: resource.clone(),
            holder,
            requester,
        })
    );

    assert_eq!(manager.lock_owner(&resource), Some(holder));

    println!("Ejercicio completado: el lock exclusivo detectó el conflicto.");
}
