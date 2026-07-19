//! Modelo educativo inicial de transacciones.
//!
//! Este módulo fija el vocabulario y las transiciones mínimas. Una transacción
//! tiene identidad (`TransactionId`), estado (`TransactionState`) y vive dentro
//! de un `TransactionManager` que registra el ciclo de vida visible. Los
//! conflictos simples quedan para pasos posteriores del capítulo.

use std::collections::BTreeMap;

/// Administrador educativo de transacciones.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionManager {
    next_transaction_id: TransactionId,
    transactions: BTreeMap<TransactionId, TransactionState>,
}

impl TransactionManager {
    /// Crea un administrador sin transacciones registradas.
    pub fn new() -> Self {
        Self {
            next_transaction_id: TransactionId::new(1),
            transactions: BTreeMap::new(),
        }
    }

    /// Devuelve `true` cuando no hay transacciones registradas.
    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }

    /// Número de transacciones conocidas por el administrador.
    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    /// Siguiente identificador que se asignaría al registrar una transacción.
    pub fn next_transaction_id(&self) -> TransactionId {
        self.next_transaction_id
    }

    /// Registra una transacción con un estado inicial explícito.
    pub fn register(&mut self, state: TransactionState) -> Result<TransactionId, TransactionError> {
        let transaction_id = self.next_transaction_id;
        self.transactions.insert(transaction_id, state);
        self.next_transaction_id = transaction_id.next();

        Ok(transaction_id)
    }

    /// Abre una transacción nueva en estado activo.
    pub fn begin(&mut self) -> Result<TransactionId, TransactionError> {
        self.register(TransactionState::Active)
    }

    /// Cierra una transacción activa aceptando sus cambios.
    pub fn commit(&mut self, transaction_id: TransactionId) -> Result<(), TransactionError> {
        self.transition(transaction_id, TransactionState::Committed)
    }

    /// Cierra una transacción activa descartando sus cambios.
    pub fn rollback(&mut self, transaction_id: TransactionId) -> Result<(), TransactionError> {
        self.transition(transaction_id, TransactionState::RolledBack)
    }

    /// Devuelve el estado de una transacción conocida.
    pub fn state(&self, transaction_id: TransactionId) -> Option<TransactionState> {
        self.transactions.get(&transaction_id).copied()
    }

    fn transition(
        &mut self,
        transaction_id: TransactionId,
        requested: TransactionState,
    ) -> Result<(), TransactionError> {
        let state = self
            .transactions
            .get_mut(&transaction_id)
            .ok_or(TransactionError::UnknownTransaction(transaction_id))?;

        if *state != TransactionState::Active {
            return Err(TransactionError::InvalidStateTransition {
                transaction_id,
                from: *state,
                requested,
            });
        }

        *state = requested;
        Ok(())
    }
}

impl Default for TransactionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Identificador lógico de una transacción.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransactionId(u64);

impl TransactionId {
    /// Crea un identificador lógico de transacción.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico del identificador.
    pub const fn value(self) -> u64 {
        self.0
    }

    const fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

/// Estado visible de una transacción educativa.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionState {
    /// La transacción está abierta y puede recibir trabajo.
    Active,
    /// La transacción terminó aceptando sus cambios.
    Committed,
    /// La transacción terminó descartando sus cambios.
    RolledBack,
}

impl TransactionState {
    /// Nombre estable del estado para documentación y ejemplos.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Committed => "committed",
            Self::RolledBack => "rolled_back",
        }
    }
}

/// Errores del modelo educativo de transacciones.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionError {
    /// Una transacción buscada no existe en el administrador.
    UnknownTransaction(TransactionId),
    /// Una transacción conocida recibió una transición no permitida.
    InvalidStateTransition {
        /// Transacción que recibió la transición.
        transaction_id: TransactionId,
        /// Estado actual de la transacción.
        from: TransactionState,
        /// Estado terminal solicitado.
        requested: TransactionState,
    },
    /// Una invariante interna fue violada.
    InvariantViolation(&'static str),
}
