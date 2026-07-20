use rust_database_internals::lsm_tree::{CompactionPlan, LsmKey, LsmTree, LsmValue, SegmentId};

fn main() {
    let mut tree = LsmTree::new(4).expect("la capacidad debe ser válida");

    tree.write(LsmKey::new(1), LsmValue::from("old"))
        .expect("la escritura debe caber");
    tree.write(LsmKey::new(2), LsmValue::from("two"))
        .expect("la escritura debe caber");
    tree.flush_to_sstable(SegmentId::new(1))
        .expect("primer flush válido");

    tree.write(LsmKey::new(1), LsmValue::from("new"))
        .expect("la escritura debe caber");
    tree.write(LsmKey::new(3), LsmValue::from("three"))
        .expect("la escritura debe caber");
    tree.flush_to_sstable(SegmentId::new(2))
        .expect("segundo flush válido");

    let plan = CompactionPlan::new(
        vec![SegmentId::new(1), SegmentId::new(2)],
        SegmentId::new(3),
    )
    .expect("plan válido");

    tree.compact(plan).expect("compaction válida");

    assert_eq!(tree.segments().len(), 1);
    assert_eq!(tree.segments()[0].segment_id(), SegmentId::new(3));
    assert_eq!(tree.search(LsmKey::new(1)), Some(LsmValue::from("new")));

    println!("solución nivel 3: compaction conserva el valor más reciente");
}
