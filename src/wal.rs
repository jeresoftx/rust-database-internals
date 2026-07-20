//! Registros educativos de Write-Ahead Log.
//!
//! Este módulo fija el vocabulario mínimo para hablar de WAL: cada registro
//! tiene un LSN, una transacción lógica y una operación. Todavía no implementa
//! un log append-only ni recovery; esos pasos vienen después.

/// Log append-only que preserva registros WAL en orden de LSN.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteAheadLog {
    next_lsn: LogSequenceNumber,
    records: Vec<LogRecord>,
}

impl WriteAheadLog {
    /// Crea un log vacío con LSN inicial `1`.
    pub const fn new() -> Self {
        Self {
            next_lsn: LogSequenceNumber::new(1),
            records: Vec::new(),
        }
    }

    /// Devuelve `true` cuando no hay registros.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Número de registros almacenados.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Siguiente LSN que recibirá un registro nuevo.
    pub const fn next_lsn(&self) -> LogSequenceNumber {
        self.next_lsn
    }

    /// Último LSN escrito, si existe.
    pub fn last_lsn(&self) -> Option<LogSequenceNumber> {
        self.records.last().map(LogRecord::lsn)
    }

    /// Registros en orden de append.
    pub fn records(&self) -> &[LogRecord] {
        &self.records
    }

    /// Itera los registros en orden de append.
    pub fn iter(&self) -> impl Iterator<Item = &LogRecord> {
        self.records.iter()
    }

    /// Agrega una operación y asigna el siguiente LSN disponible.
    pub fn append(
        &mut self,
        transaction_id: WalTransactionId,
        operation: LogOperation,
    ) -> LogSequenceNumber {
        let lsn = self.next_lsn;
        let record = LogRecord::new(lsn, transaction_id, operation);
        self.records.push(record);
        self.next_lsn = lsn.next();
        lsn
    }

    /// Agrega un registro existente si su LSN coincide con el siguiente.
    pub fn append_record(&mut self, record: LogRecord) -> Result<(), WalError> {
        if record.lsn() != self.next_lsn {
            return Err(WalError::UnexpectedLsn {
                expected: self.next_lsn,
                actual: record.lsn(),
            });
        }

        self.next_lsn = self.next_lsn.next();
        self.records.push(record);
        Ok(())
    }

    /// Agrega un registro `Begin`.
    pub fn append_begin(&mut self, transaction_id: WalTransactionId) -> LogSequenceNumber {
        self.append(transaction_id, LogOperation::Begin)
    }

    /// Agrega un registro `Commit`.
    pub fn append_commit(&mut self, transaction_id: WalTransactionId) -> LogSequenceNumber {
        self.append(transaction_id, LogOperation::Commit)
    }

    /// Agrega un registro `Rollback`.
    pub fn append_rollback(&mut self, transaction_id: WalTransactionId) -> LogSequenceNumber {
        self.append(transaction_id, LogOperation::Rollback)
    }
}

impl Default for WriteAheadLog {
    fn default() -> Self {
        Self::new()
    }
}

/// Registro individual de Write-Ahead Log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogRecord {
    lsn: LogSequenceNumber,
    transaction_id: WalTransactionId,
    operation: LogOperation,
}

impl LogRecord {
    /// Crea un registro WAL con metadatos explícitos.
    pub const fn new(
        lsn: LogSequenceNumber,
        transaction_id: WalTransactionId,
        operation: LogOperation,
    ) -> Self {
        Self {
            lsn,
            transaction_id,
            operation,
        }
    }

    /// Crea un registro de inicio de transacción.
    pub const fn begin(lsn: LogSequenceNumber, transaction_id: WalTransactionId) -> Self {
        Self::new(lsn, transaction_id, LogOperation::Begin)
    }

    /// Crea un registro de commit.
    pub const fn commit(lsn: LogSequenceNumber, transaction_id: WalTransactionId) -> Self {
        Self::new(lsn, transaction_id, LogOperation::Commit)
    }

    /// Crea un registro de rollback.
    pub const fn rollback(lsn: LogSequenceNumber, transaction_id: WalTransactionId) -> Self {
        Self::new(lsn, transaction_id, LogOperation::Rollback)
    }

    /// LSN asignado al registro.
    pub const fn lsn(&self) -> LogSequenceNumber {
        self.lsn
    }

    /// Transacción lógica asociada al registro.
    pub const fn transaction_id(&self) -> WalTransactionId {
        self.transaction_id
    }

    /// Operación registrada.
    pub const fn operation(&self) -> &LogOperation {
        &self.operation
    }

    /// Devuelve `true` si el registro contiene información para rehacer.
    pub const fn is_redoable(&self) -> bool {
        self.operation.is_redoable()
    }

    /// Devuelve `true` si el registro contiene información para deshacer.
    pub const fn is_undoable(&self) -> bool {
        self.operation.is_undoable()
    }
}

/// Operación educativa registrada en WAL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogOperation {
    /// Inicio de una transacción.
    Begin,
    /// Cambio de una página lógica con imagen anterior y posterior.
    Update {
        /// Página lógica modificada.
        page_id: PageId,
        /// Imagen antes del cambio.
        before: PageImage,
        /// Imagen después del cambio.
        after: PageImage,
    },
    /// Cierre exitoso de la transacción.
    Commit,
    /// Cierre descartando cambios.
    Rollback,
}

impl LogOperation {
    /// Crea una operación de actualización con delta observable.
    pub fn update(page_id: PageId, before: PageImage, after: PageImage) -> Result<Self, WalError> {
        if before == after {
            return Err(WalError::NoPageChange);
        }

        Ok(Self::Update {
            page_id,
            before,
            after,
        })
    }

    /// Nombre estable de la operación para documentación y ejemplos.
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Begin => "begin",
            Self::Update { .. } => "update",
            Self::Commit => "commit",
            Self::Rollback => "rollback",
        }
    }

    /// Devuelve `true` si la operación puede rehacerse.
    pub const fn is_redoable(&self) -> bool {
        matches!(self, Self::Update { .. })
    }

    /// Devuelve `true` si la operación puede deshacerse.
    pub const fn is_undoable(&self) -> bool {
        matches!(self, Self::Update { .. })
    }
}

/// Log Sequence Number educativo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LogSequenceNumber(u64);

impl LogSequenceNumber {
    /// Crea un LSN.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico del LSN.
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Siguiente LSN lógico.
    pub const fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

/// Identificador lógico de transacción para WAL.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WalTransactionId(u64);

impl WalTransactionId {
    /// Crea un identificador lógico de transacción.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico del identificador.
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// Página lógica afectada por una operación WAL.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PageId(String);

impl PageId {
    /// Crea un identificador de página no vacío.
    pub fn new(value: impl Into<String>) -> Result<Self, WalError> {
        let value = normalize(value.into());

        if value.is_empty() {
            return Err(WalError::BlankPageId);
        }

        Ok(Self(value))
    }

    /// Devuelve el nombre estable de la página.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Imagen educativa de página antes o después de un cambio.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageImage(String);

impl PageImage {
    /// Crea una imagen de página no vacía.
    pub fn new(value: impl Into<String>) -> Result<Self, WalError> {
        let value = normalize(value.into());

        if value.is_empty() {
            return Err(WalError::BlankPageImage);
        }

        Ok(Self(value))
    }

    /// Devuelve el contenido educativo de la imagen.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Errores del modelo educativo de WAL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalError {
    /// El identificador de página está vacío.
    BlankPageId,
    /// La imagen de página está vacía.
    BlankPageImage,
    /// Una actualización sin cambio observable no debe registrarse como delta.
    NoPageChange,
    /// Un registro externo no coincide con el siguiente LSN esperado.
    UnexpectedLsn {
        /// Siguiente LSN esperado por el log.
        expected: LogSequenceNumber,
        /// LSN encontrado en el registro.
        actual: LogSequenceNumber,
    },
}

fn normalize(value: String) -> String {
    value.trim().to_owned()
}
