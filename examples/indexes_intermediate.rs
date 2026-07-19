use rust_database_internals::indexes::{
    IndexEntries, IndexEntryKey, IndexUniqueness, PrimaryKeyValue,
};

fn main() {
    let mut country_index = IndexEntries::new(IndexUniqueness::NonUnique);

    for (country, customer_id) in [("MX", "1001"), ("MX", "1002"), ("US", "1003")] {
        country_index
            .insert(
                IndexEntryKey::from(country),
                PrimaryKeyValue::from(customer_id),
            )
            .expect("un índice no único acepta varios clientes por país");
    }

    let candidates = country_index.primary_keys_for(&IndexEntryKey::from("MX"));

    assert_eq!(
        candidates,
        vec![PrimaryKeyValue::from("1001"), PrimaryKeyValue::from("1002")]
    );

    println!("country=MX deja {} filas candidatas", candidates.len());
}
