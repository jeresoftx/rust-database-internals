use rust_database_internals::indexes::{
    IndexEntries, IndexEntryKey, IndexUniqueness, PrimaryKeyValue, SelectivityClass,
};

fn main() {
    let mut status_index = IndexEntries::new(IndexUniqueness::NonUnique);

    for customer_id in ["1001", "1002", "1003", "1004"] {
        status_index
            .insert(
                IndexEntryKey::from("active"),
                PrimaryKeyValue::from(customer_id),
            )
            .expect("un índice no único acepta estados repetidos");
    }

    let selectivity = status_index.selectivity();

    assert_eq!(selectivity.distinct_keys(), 1);
    assert_eq!(selectivity.indexed_rows(), 4);
    assert_eq!(selectivity.class(), SelectivityClass::Low);

    println!(
        "status tiene selectividad {:?}: ratio={:.2}, candidatos para active={}",
        selectivity.class(),
        selectivity.ratio(),
        status_index.estimated_candidates_for(&IndexEntryKey::from("active"))
    );
}
