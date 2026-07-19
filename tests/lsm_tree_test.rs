use rust_database_internals::lsm_tree::{
    CompactionPlan, LsmKey, LsmTreeError, LsmValue, MemTable, SSTable, SegmentId,
};

#[test]
fn memtable_declares_capacity_and_starts_empty() {
    let memtable = MemTable::new(4).expect("una capacidad positiva debe crear una MemTable");

    assert!(memtable.is_empty());
    assert_eq!(memtable.len(), 0);
    assert_eq!(memtable.max_entries(), 4);
    assert!(memtable.can_accept_entry());
}

#[test]
fn memtable_rejects_zero_capacity() {
    assert_eq!(
        MemTable::new(0),
        Err(LsmTreeError::InvalidMemTableCapacity { max_entries: 0 })
    );
}

#[test]
fn memtable_writes_entries_in_key_order() {
    let mut memtable = MemTable::new(4).expect("una capacidad positiva debe crear una MemTable");

    assert_eq!(
        memtable.write(LsmKey::new(30), LsmValue::from("thirty")),
        Ok(())
    );
    assert_eq!(
        memtable.write(LsmKey::new(10), LsmValue::from("ten")),
        Ok(())
    );
    assert_eq!(
        memtable.write(LsmKey::new(20), LsmValue::from("twenty")),
        Ok(())
    );

    assert!(!memtable.is_empty());
    assert_eq!(memtable.len(), 3);
    assert!(memtable.can_accept_entry());
    assert_eq!(
        memtable.entries(),
        vec![
            (LsmKey::new(10), LsmValue::from("ten")),
            (LsmKey::new(20), LsmValue::from("twenty")),
            (LsmKey::new(30), LsmValue::from("thirty")),
        ]
    );
}

#[test]
fn memtable_overwrites_existing_key_without_growing() {
    let mut memtable = MemTable::new(1).expect("una capacidad positiva debe crear una MemTable");

    assert_eq!(
        memtable.write(LsmKey::new(7), LsmValue::from("first")),
        Ok(())
    );
    assert_eq!(
        memtable.write(LsmKey::new(7), LsmValue::from("second")),
        Ok(())
    );

    assert_eq!(memtable.len(), 1);
    assert!(!memtable.can_accept_entry());
    assert_eq!(
        memtable.entries(),
        vec![(LsmKey::new(7), LsmValue::from("second"))]
    );
}

#[test]
fn memtable_rejects_new_key_when_full() {
    let mut memtable = MemTable::new(1).expect("una capacidad positiva debe crear una MemTable");

    assert_eq!(
        memtable.write(LsmKey::new(1), LsmValue::from("one")),
        Ok(())
    );

    assert_eq!(
        memtable.write(LsmKey::new(2), LsmValue::from("two")),
        Err(LsmTreeError::MemTableFull { max_entries: 1 })
    );
    assert_eq!(memtable.len(), 1);
    assert_eq!(
        memtable.entries(),
        vec![(LsmKey::new(1), LsmValue::from("one"))]
    );
}

#[test]
fn sstable_declares_segment_identity_and_key_count() {
    let segment = SegmentId::new(7);
    let sstable = SSTable::new(segment, 128);

    assert_eq!(segment.value(), 7);
    assert_eq!(sstable.segment_id(), segment);
    assert_eq!(sstable.key_count(), 128);
    assert!(!sstable.is_empty());
}

#[test]
fn compaction_plan_requires_at_least_one_input_segment() {
    let result = CompactionPlan::new(Vec::new(), SegmentId::new(10));

    assert_eq!(result, Err(LsmTreeError::EmptyCompactionInput));
}

#[test]
fn compaction_plan_rejects_duplicate_input_segments() {
    let segment = SegmentId::new(1);

    let result = CompactionPlan::new(vec![segment, segment], SegmentId::new(2));

    assert_eq!(result, Err(LsmTreeError::DuplicateCompactionInput(segment)));
}

#[test]
fn compaction_plan_rejects_output_that_conflicts_with_inputs() {
    let output = SegmentId::new(2);

    let result = CompactionPlan::new(vec![SegmentId::new(1), output], output);

    assert_eq!(result, Err(LsmTreeError::OutputSegmentConflicts(output)));
}

#[test]
fn compaction_plan_keeps_input_order_and_output_segment() {
    let inputs = vec![SegmentId::new(1), SegmentId::new(2), SegmentId::new(3)];
    let output = SegmentId::new(4);

    let plan = CompactionPlan::new(inputs.clone(), output)
        .expect("entradas distintas y salida nueva deben formar un plan válido");

    assert_eq!(plan.input_segments(), inputs.as_slice());
    assert_eq!(plan.output_segment(), output);
}
