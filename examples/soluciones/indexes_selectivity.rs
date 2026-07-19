use rust_database_internals::indexes::{
    IndexEntries, IndexEntryKey, IndexUniqueness, PrimaryKeyValue, SelectivityClass,
};

fn main() {
    let mut country_index = IndexEntries::new(IndexUniqueness::NonUnique);

    for (country, customer_id) in [("MX", "1001"), ("MX", "1002"), ("US", "1003")] {
        country_index
            .insert(
                IndexEntryKey::from(country),
                PrimaryKeyValue::from(customer_id),
            )
            .expect("un índice no único acepta países repetidos");
    }

    let selectivity = country_index.selectivity();

    assert_eq!(selectivity.distinct_keys(), 2);
    assert_eq!(selectivity.indexed_rows(), 3);
    assert_eq!(selectivity.class(), SelectivityClass::Medium);

    println!(
        "country selectivity: ratio={:.2}, class={:?}",
        selectivity.ratio(),
        selectivity.class()
    );
}
