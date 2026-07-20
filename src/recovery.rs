//! Modelo educativo de recovery después de una caída.
//!
//! Este módulo inicia recovery en el punto más pequeño posible: observar el WAL
//! y decidir qué transacciones necesitan redo o undo después de un crash.
//! Todavía no reproduce el log sobre páginas; ese paso pertenece al replay.

use std::collections::BTreeSet;

use crate::wal::{LogOperation, WalTransactionId, WriteAheadLog};

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
}
