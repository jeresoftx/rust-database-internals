//! Modelo educativo de recovery después de una caída.
//!
//! Este módulo observa el WAL, decide qué transacciones necesitan redo o undo
//! después de un crash y reproduce ese plan sobre un almacén educativo de
//! páginas.

use std::collections::BTreeSet;

use crate::wal::{LogOperation, PageStore, WalError, WalTransactionId, WriteAheadLog};

/// Plan de recovery derivado de los registros WAL disponibles al reiniciar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPlan {
    redo_transactions: Vec<WalTransactionId>,
    undo_transactions: Vec<WalTransactionId>,
}

impl RecoveryPlan {
    /// Construye un plan de recovery a partir de la historia escrita en WAL.
    pub fn from_wal(log: &WriteAheadLog) -> Self {
        let mut dirty_transactions = BTreeSet::new();
        let mut committed_transactions = BTreeSet::new();

        for record in log.iter() {
            let transaction_id = record.transaction_id();

            match record.operation() {
                LogOperation::Begin => {}
                LogOperation::Update { .. } => {
                    dirty_transactions.insert(transaction_id);
                }
                LogOperation::Commit => {
                    if dirty_transactions.remove(&transaction_id) {
                        committed_transactions.insert(transaction_id);
                    }
                }
                LogOperation::Rollback => {
                    dirty_transactions.remove(&transaction_id);
                    committed_transactions.remove(&transaction_id);
                }
            }
        }

        Self {
            redo_transactions: committed_transactions.into_iter().collect(),
            undo_transactions: dirty_transactions.into_iter().collect(),
        }
    }

    /// Devuelve `true` cuando no hay trabajo de recovery pendiente.
    pub fn is_empty(&self) -> bool {
        self.redo_transactions.is_empty() && self.undo_transactions.is_empty()
    }

    /// Transacciones confirmadas que deben considerarse para redo.
    pub fn redo_transactions(&self) -> &[WalTransactionId] {
        &self.redo_transactions
    }

    /// Transacciones con cambios no confirmados que deben considerarse para undo.
    pub fn undo_transactions(&self) -> &[WalTransactionId] {
        &self.undo_transactions
    }

    /// Devuelve `true` si la transacción aparece como candidata a redo.
    pub fn requires_redo(&self, transaction_id: WalTransactionId) -> bool {
        self.redo_transactions.contains(&transaction_id)
    }

    /// Devuelve `true` si la transacción aparece como candidata a undo.
    pub fn requires_undo(&self, transaction_id: WalTransactionId) -> bool {
        self.undo_transactions.contains(&transaction_id)
    }

    /// Reproduce el WAL sobre un almacén de páginas educativo.
    pub fn replay(
        &self,
        log: &WriteAheadLog,
        store: &mut PageStore,
    ) -> Result<RecoveryReport, WalError> {
        let mut report = RecoveryReport::default();

        for record in log.iter() {
            if record.is_redoable() && self.requires_redo(record.transaction_id()) {
                store.redo(record)?;
                report.redone_records += 1;
            }
        }

        for record in log.records().iter().rev() {
            if record.is_undoable() && self.requires_undo(record.transaction_id()) {
                store.undo(record)?;
                report.undone_records += 1;
            }
        }

        Ok(report)
    }
}

/// Resultado observable de reproducir un plan de recovery.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RecoveryReport {
    redone_records: usize,
    undone_records: usize,
}

impl RecoveryReport {
    /// Cantidad de registros aplicados con redo.
    pub const fn redone_records(self) -> usize {
        self.redone_records
    }

    /// Cantidad de registros aplicados con undo.
    pub const fn undone_records(self) -> usize {
        self.undone_records
    }
}
