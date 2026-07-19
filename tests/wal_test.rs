use rust_database_internals::wal::{
    LogOperation, LogRecord, LogSequenceNumber, PageId, PageImage, WalError, WalTransactionId,
    WriteAheadLog,
};

#[test]
fn log_sequence_number_exposes_value_and_next() {
    let lsn = LogSequenceNumber::new(42);

    assert_eq!(lsn.value(), 42);
    assert_eq!(lsn.next(), LogSequenceNumber::new(43));
}

#[test]
fn wal_transaction_id_exposes_value() {
    let transaction_id = WalTransactionId::new(7);

    assert_eq!(transaction_id.value(), 7);
}

#[test]
fn page_id_rejects_blank_text() {
    assert_eq!(PageId::new("   "), Err(WalError::BlankPageId));
}

#[test]
fn page_id_trims_and_exposes_stable_name() {
    let page_id = PageId::new(" heap/accounts/0001 ").expect("page id válido");

    assert_eq!(page_id.as_str(), "heap/accounts/0001");
}

#[test]
fn page_image_rejects_blank_text() {
    assert_eq!(PageImage::new("   "), Err(WalError::BlankPageImage));
}

#[test]
fn page_image_trims_and_exposes_payload() {
    let image = PageImage::new(" saldo=100 ").expect("imagen válida");

    assert_eq!(image.as_str(), "saldo=100");
}

#[test]
fn update_operation_rejects_noop_page_change() {
    let page_id = PageId::new("heap/accounts/0001").expect("page id válido");
    let before = PageImage::new("saldo=100").expect("imagen válida");
    let after = PageImage::new("saldo=100").expect("imagen válida");

    assert_eq!(
        LogOperation::update(page_id, before, after),
        Err(WalError::NoPageChange)
    );
}

#[test]
fn begin_record_exposes_lsn_transaction_and_operation() {
    let record = LogRecord::begin(LogSequenceNumber::new(1), WalTransactionId::new(10));

    assert_eq!(record.lsn(), LogSequenceNumber::new(1));
    assert_eq!(record.transaction_id(), WalTransactionId::new(10));
    assert_eq!(record.operation(), &LogOperation::Begin);
    assert_eq!(record.operation().name(), "begin");
    assert!(!record.is_redoable());
    assert!(!record.is_undoable());
}

#[test]
fn update_record_exposes_page_delta_and_is_redoable_and_undoable() {
    let page_id = PageId::new("heap/accounts/0001").expect("page id válido");
    let before = PageImage::new("saldo=100").expect("imagen válida");
    let after = PageImage::new("saldo=120").expect("imagen válida");
    let operation = LogOperation::update(page_id.clone(), before.clone(), after.clone())
        .expect("update válido");

    let record = LogRecord::new(
        LogSequenceNumber::new(2),
        WalTransactionId::new(10),
        operation,
    );

    assert_eq!(record.lsn(), LogSequenceNumber::new(2));
    assert_eq!(record.transaction_id(), WalTransactionId::new(10));
    assert_eq!(record.operation().name(), "update");
    assert!(record.is_redoable());
    assert!(record.is_undoable());

    match record.operation() {
        LogOperation::Update {
            page_id: stored_page,
            before: stored_before,
            after: stored_after,
        } => {
            assert_eq!(stored_page, &page_id);
            assert_eq!(stored_before, &before);
            assert_eq!(stored_after, &after);
        }
        other => panic!("se esperaba update, se obtuvo {other:?}"),
    }
}

#[test]
fn commit_record_is_a_terminal_operation() {
    let record = LogRecord::commit(LogSequenceNumber::new(3), WalTransactionId::new(10));

    assert_eq!(record.operation(), &LogOperation::Commit);
    assert_eq!(record.operation().name(), "commit");
    assert!(!record.is_redoable());
    assert!(!record.is_undoable());
}

#[test]
fn rollback_record_is_a_terminal_operation() {
    let record = LogRecord::rollback(LogSequenceNumber::new(4), WalTransactionId::new(10));

    assert_eq!(record.operation(), &LogOperation::Rollback);
    assert_eq!(record.operation().name(), "rollback");
    assert!(!record.is_redoable());
    assert!(!record.is_undoable());
}

#[test]
fn write_ahead_log_starts_empty() {
    let log = WriteAheadLog::new();

    assert!(log.is_empty());
    assert_eq!(log.len(), 0);
    assert_eq!(log.next_lsn(), LogSequenceNumber::new(1));
    assert_eq!(log.last_lsn(), None);
    assert!(log.records().is_empty());
}

#[test]
fn write_ahead_log_appends_records_with_monotonic_lsn() {
    let mut log = WriteAheadLog::new();
    let transaction_id = WalTransactionId::new(10);

    let begin_lsn = log.append_begin(transaction_id);
    let commit_lsn = log.append_commit(transaction_id);

    assert_eq!(begin_lsn, LogSequenceNumber::new(1));
    assert_eq!(commit_lsn, LogSequenceNumber::new(2));
    assert_eq!(log.next_lsn(), LogSequenceNumber::new(3));
    assert_eq!(log.last_lsn(), Some(LogSequenceNumber::new(2)));
}

#[test]
fn write_ahead_log_preserves_append_order() {
    let mut log = WriteAheadLog::new();
    let transaction_id = WalTransactionId::new(10);
    let page_id = PageId::new("heap/accounts/0001").expect("page id válido");
    let before = PageImage::new("saldo=100").expect("imagen válida");
    let after = PageImage::new("saldo=120").expect("imagen válida");
    let update = LogOperation::update(page_id, before, after).expect("update válido");

    log.append_begin(transaction_id);
    log.append(transaction_id, update);
    log.append_commit(transaction_id);

    let operations: Vec<&str> = log.iter().map(|record| record.operation().name()).collect();

    assert_eq!(operations, vec!["begin", "update", "commit"]);
}

#[test]
fn write_ahead_log_can_append_existing_record_when_lsn_matches_next() {
    let mut log = WriteAheadLog::new();
    let record = LogRecord::begin(LogSequenceNumber::new(1), WalTransactionId::new(10));

    log.append_record(record.clone())
        .expect("registro con LSN esperado debe entrar");

    assert_eq!(log.records(), &[record]);
    assert_eq!(log.next_lsn(), LogSequenceNumber::new(2));
}

#[test]
fn write_ahead_log_rejects_existing_record_with_unexpected_lsn() {
    let mut log = WriteAheadLog::new();
    let record = LogRecord::begin(LogSequenceNumber::new(2), WalTransactionId::new(10));

    assert_eq!(
        log.append_record(record),
        Err(WalError::UnexpectedLsn {
            expected: LogSequenceNumber::new(1),
            actual: LogSequenceNumber::new(2),
        })
    );
    assert!(log.is_empty());
}
