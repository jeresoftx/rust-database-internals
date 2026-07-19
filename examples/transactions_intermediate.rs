use rust_database_internals::transactions::{ResourceId, TransactionError, TransactionManager};

fn main() {
    let mut manager = TransactionManager::new();
    let holder = manager
        .begin()
        .expect("abrir la primera transacción debe producir un id");
    let requester = manager
        .begin()
        .expect("abrir la segunda transacción debe producir un id");
    let account = ResourceId::new("accounts/42").expect("recurso válido");

    manager
        .lock_exclusive(holder, account.clone())
        .expect("la primera transacción toma el recurso");

    assert_eq!(
        manager.lock_exclusive(requester, account.clone()),
        Err(TransactionError::ResourceConflict {
            resource: account.clone(),
            holder,
            requester,
        })
    );

    println!(
        "{} está protegido por la transacción {}.",
        account.as_str(),
        holder.value()
    );
}
