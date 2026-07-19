use rust_database_internals::lsm_tree::{
    CompactionPlan, LsmKey, LsmTree, LsmTreeError, LsmValue, MemTable, SSTable, SegmentId,
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
fn memtable_flushes_entries_into_immutable_sstable() {
    let mut memtable = MemTable::new(4).expect("una capacidad positiva debe crear una MemTable");
    memtable
        .write(LsmKey::new(30), LsmValue::from("thirty"))
        .expect("la escritura debe caber");
    memtable
        .write(LsmKey::new(10), LsmValue::from("ten"))
        .expect("la escritura debe caber");

    let sstable = memtable
        .flush_to_sstable(SegmentId::new(9))
        .expect("una MemTable con entradas debe producir una SSTable");

    assert!(memtable.is_empty());
    assert_eq!(memtable.len(), 0);
    assert_eq!(sstable.segment_id(), SegmentId::new(9));
    assert_eq!(sstable.key_count(), 2);
    assert_eq!(
        sstable.entries(),
        vec![
            (LsmKey::new(10), LsmValue::from("ten")),
            (LsmKey::new(30), LsmValue::from("thirty")),
        ]
    );
}

#[test]
fn memtable_flush_rejects_empty_memtable() {
    let mut memtable = MemTable::new(4).expect("una capacidad positiva debe crear una MemTable");

    assert_eq!(
        memtable.flush_to_sstable(SegmentId::new(1)),
        Err(LsmTreeError::EmptyMemTableFlush)
    );
}

#[test]
fn flushed_sstable_is_not_changed_by_later_memtable_writes() {
    let mut memtable = MemTable::new(4).expect("una capacidad positiva debe crear una MemTable");
    memtable
        .write(LsmKey::new(1), LsmValue::from("before"))
        .expect("la escritura debe caber");
    let sstable = memtable
        .flush_to_sstable(SegmentId::new(11))
        .expect("una MemTable con entradas debe producir una SSTable");

    memtable
        .write(LsmKey::new(1), LsmValue::from("after"))
        .expect("la escritura posterior debe caber");

    assert_eq!(
        sstable.entries(),
        vec![(LsmKey::new(1), LsmValue::from("before"))]
    );
    assert_eq!(
        memtable.entries(),
        vec![(LsmKey::new(1), LsmValue::from("after"))]
    );
}

#[test]
fn lsm_tree_searches_memtable_before_segments() {
    let mut tree = LsmTree::new(4).expect("una capacidad positiva debe crear un LSM Tree");
    tree.write(LsmKey::new(1), LsmValue::from("older"))
        .expect("la escritura debe caber");
    tree.flush_to_sstable(SegmentId::new(1))
        .expect("el flush debe crear un segmento");
    tree.write(LsmKey::new(1), LsmValue::from("newer"))
        .expect("la escritura en memoria debe caber");

    assert_eq!(tree.search(LsmKey::new(1)), Some(LsmValue::from("newer")));
}

#[test]
fn lsm_tree_searches_newest_sstable_before_older_segments() {
    let mut tree = LsmTree::new(4).expect("una capacidad positiva debe crear un LSM Tree");
    tree.write(LsmKey::new(5), LsmValue::from("old"))
        .expect("la escritura debe caber");
    tree.flush_to_sstable(SegmentId::new(1))
        .expect("el flush debe crear un segmento");
    tree.write(LsmKey::new(5), LsmValue::from("new"))
        .expect("la escritura debe caber");
    tree.flush_to_sstable(SegmentId::new(2))
        .expect("el flush debe crear un segmento");

    assert_eq!(tree.search(LsmKey::new(5)), Some(LsmValue::from("new")));
    assert_eq!(tree.segments().len(), 2);
}

#[test]
fn lsm_tree_search_returns_none_when_key_is_absent() {
    let mut tree = LsmTree::new(4).expect("una capacidad positiva debe crear un LSM Tree");
    tree.write(LsmKey::new(9), LsmValue::from("present"))
        .expect("la escritura debe caber");
    tree.flush_to_sstable(SegmentId::new(1))
        .expect("el flush debe crear un segmento");

    assert_eq!(tree.search(LsmKey::new(10)), None);
}

#[test]
fn lsm_tree_compaction_keeps_newest_value_for_duplicate_keys() {
    let mut tree = LsmTree::new(4).expect("una capacidad positiva debe crear un LSM Tree");
    tree.write(LsmKey::new(1), LsmValue::from("old"))
        .expect("la escritura debe caber");
    tree.write(LsmKey::new(2), LsmValue::from("two"))
        .expect("la escritura debe caber");
    tree.flush_to_sstable(SegmentId::new(1))
        .expect("el primer flush debe crear un segmento");
    tree.write(LsmKey::new(1), LsmValue::from("new"))
        .expect("la escritura debe caber");
    tree.write(LsmKey::new(3), LsmValue::from("three"))
        .expect("la escritura debe caber");
    tree.flush_to_sstable(SegmentId::new(2))
        .expect("el segundo flush debe crear un segmento");

    let plan = CompactionPlan::new(
        vec![SegmentId::new(1), SegmentId::new(2)],
        SegmentId::new(3),
    )
    .expect("un plan con entradas distintas debe ser válido");

    assert_eq!(tree.compact(plan), Ok(()));

    assert_eq!(tree.segments().len(), 1);
    assert_eq!(tree.segments()[0].segment_id(), SegmentId::new(3));
    assert_eq!(
        tree.segments()[0].entries(),
        vec![
            (LsmKey::new(1), LsmValue::from("new")),
            (LsmKey::new(2), LsmValue::from("two")),
            (LsmKey::new(3), LsmValue::from("three")),
        ]
    );
    assert_eq!(tree.search(LsmKey::new(1)), Some(LsmValue::from("new")));
}

#[test]
fn lsm_tree_compaction_keeps_segments_outside_plan() {
    let mut tree = LsmTree::new(4).expect("una capacidad positiva debe crear un LSM Tree");
    tree.write(LsmKey::new(1), LsmValue::from("one"))
        .expect("la escritura debe caber");
    tree.flush_to_sstable(SegmentId::new(1))
        .expect("el primer flush debe crear un segmento");
    tree.write(LsmKey::new(2), LsmValue::from("two"))
        .expect("la escritura debe caber");
    tree.flush_to_sstable(SegmentId::new(2))
        .expect("el segundo flush debe crear un segmento");
    tree.write(LsmKey::new(3), LsmValue::from("three"))
        .expect("la escritura debe caber");
    tree.flush_to_sstable(SegmentId::new(3))
        .expect("el tercer flush debe crear un segmento");

    let plan = CompactionPlan::new(
        vec![SegmentId::new(1), SegmentId::new(3)],
        SegmentId::new(4),
    )
    .expect("un plan con entradas distintas debe ser válido");

    assert_eq!(tree.compact(plan), Ok(()));

    let segment_ids: Vec<_> = tree
        .segments()
        .iter()
        .map(|segment| segment.segment_id())
        .collect();
    assert_eq!(segment_ids, vec![SegmentId::new(2), SegmentId::new(4)]);
    assert_eq!(tree.search(LsmKey::new(1)), Some(LsmValue::from("one")));
    assert_eq!(tree.search(LsmKey::new(2)), Some(LsmValue::from("two")));
    assert_eq!(tree.search(LsmKey::new(3)), Some(LsmValue::from("three")));
}

#[test]
fn lsm_tree_compaction_rejects_missing_segment() {
    let mut tree = LsmTree::new(4).expect("una capacidad positiva debe crear un LSM Tree");
    tree.write(LsmKey::new(1), LsmValue::from("one"))
        .expect("la escritura debe caber");
    tree.flush_to_sstable(SegmentId::new(1))
        .expect("el flush debe crear un segmento");

    let plan = CompactionPlan::new(
        vec![SegmentId::new(1), SegmentId::new(99)],
        SegmentId::new(2),
    )
    .expect("la validez estructural del plan no depende del árbol");

    assert_eq!(
        tree.compact(plan),
        Err(LsmTreeError::MissingSegment(SegmentId::new(99)))
    );
    assert_eq!(tree.segments().len(), 1);
    assert_eq!(tree.segments()[0].segment_id(), SegmentId::new(1));
}

#[test]
fn lsm_tree_compaction_rejects_existing_output_segment() {
    let mut tree = LsmTree::new(4).expect("una capacidad positiva debe crear un LSM Tree");
    tree.write(LsmKey::new(1), LsmValue::from("one"))
        .expect("la escritura debe caber");
    tree.flush_to_sstable(SegmentId::new(1))
        .expect("el primer flush debe crear un segmento");
    tree.write(LsmKey::new(2), LsmValue::from("two"))
        .expect("la escritura debe caber");
    tree.flush_to_sstable(SegmentId::new(2))
        .expect("el segundo flush debe crear un segmento");

    let plan = CompactionPlan::new(vec![SegmentId::new(1)], SegmentId::new(2))
        .expect("el plan no conoce los segmentos existentes del árbol");

    assert_eq!(
        tree.compact(plan),
        Err(LsmTreeError::OutputSegmentConflicts(SegmentId::new(2)))
    );
    assert_eq!(tree.segments().len(), 2);
    assert_eq!(tree.segments()[0].segment_id(), SegmentId::new(1));
    assert_eq!(tree.segments()[1].segment_id(), SegmentId::new(2));
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
