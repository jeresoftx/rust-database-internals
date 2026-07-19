//! Representación educativa inicial de MVCC.
//!
//! Este módulo modela versiones de registro, metadatos de visibilidad y
//! lecturas por snapshot. La meta es fijar el vocabulario: un registro lógico
//! puede tener varias versiones, cada versión nace en un timestamp lógico y
//! una lectura observa la versión visible para su snapshot.

/// Cadena de versiones para un mismo registro lógico.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionChain {
    record_id: RecordId,
    next_version_id: VersionId,
    versions: Vec<RecordVersion>,
}

impl VersionChain {
    /// Crea una cadena vacía para un registro lógico.
    pub fn new(record_id: RecordId) -> Self {
        Self {
            record_id,
            next_version_id: VersionId::new(1),
            versions: Vec::new(),
        }
    }

    /// Registro lógico al que pertenecen todas las versiones.
    pub fn record_id(&self) -> &RecordId {
        &self.record_id
    }

    /// Devuelve `true` cuando la cadena todavía no tiene versiones.
    pub fn is_empty(&self) -> bool {
        self.versions.is_empty()
    }

    /// Número de versiones registradas.
    pub fn len(&self) -> usize {
        self.versions.len()
    }

    /// Versiones almacenadas en orden de creación.
    pub fn versions(&self) -> &[RecordVersion] {
        &self.versions
    }

    /// Versión más reciente por orden de inserción educativa.
    pub fn latest(&self) -> Option<&RecordVersion> {
        self.versions.last()
    }

    /// Lee la versión visible para un snapshot.
    pub fn read(&self, snapshot: &Snapshot) -> Option<&RecordVersion> {
        self.read_at(snapshot.read_at())
    }

    /// Lee la versión visible para un timestamp lógico.
    pub fn read_at(&self, read_at: LogicalTimestamp) -> Option<&RecordVersion> {
        self.versions
            .iter()
            .rev()
            .find(|version| version.is_visible_at(read_at))
    }

    /// Agrega una nueva versión con timestamp lógico monótono.
    pub fn append(
        &mut self,
        created_at: LogicalTimestamp,
        value: RecordValue,
    ) -> Result<VersionId, MvccError> {
        if let Some(previous) = self.latest().map(RecordVersion::created_at) {
            if created_at < previous {
                return Err(MvccError::NonMonotonicTimestamp {
                    previous,
                    next: created_at,
                });
            }
        }

        let version_id = self.next_version_id;
        let version = RecordVersion::new(self.record_id.clone(), version_id, created_at, value);

        self.versions.push(version);
        self.next_version_id = version_id.next();

        Ok(version_id)
    }

    /// Cierra lógicamente la versión más reciente de la cadena.
    pub fn delete_latest_at(&mut self, deleted_at: LogicalTimestamp) -> Result<(), MvccError> {
        let latest = self
            .versions
            .last_mut()
            .ok_or(MvccError::EmptyVersionChain)?;

        latest.delete_at(deleted_at)
    }
}

/// Identificador estable de un registro lógico.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RecordId(String);

impl RecordId {
    /// Crea un identificador de registro no vacío.
    pub fn new(value: impl Into<String>) -> Result<Self, MvccError> {
        let value = normalize(value.into());

        if value.is_empty() {
            return Err(MvccError::BlankRecordId);
        }

        Ok(Self(value))
    }

    /// Devuelve el valor estable del identificador.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Valor educativo asociado a una versión de registro.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordValue(String);

impl RecordValue {
    /// Crea un valor de registro no vacío.
    pub fn new(value: impl Into<String>) -> Result<Self, MvccError> {
        let value = normalize(value.into());

        if value.is_empty() {
            return Err(MvccError::BlankRecordValue);
        }

        Ok(Self(value))
    }

    /// Devuelve el contenido almacenado por la versión.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Identificador lógico de una versión dentro de una cadena.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VersionId(u64);

impl VersionId {
    /// Crea un identificador de versión.
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

/// Tiempo lógico usado para ordenar creación y cierre de versiones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LogicalTimestamp(u64);

impl LogicalTimestamp {
    /// Crea un timestamp lógico.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico del timestamp.
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// Snapshot lógico usado por una lectura MVCC.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Snapshot {
    read_at: LogicalTimestamp,
}

impl Snapshot {
    /// Crea un snapshot para un timestamp lógico de lectura.
    pub const fn new(read_at: LogicalTimestamp) -> Self {
        Self { read_at }
    }

    /// Timestamp lógico que delimita qué versiones puede observar la lectura.
    pub const fn read_at(self) -> LogicalTimestamp {
        self.read_at
    }
}

/// Resultado explícito de evaluar una versión contra un timestamp lógico.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisibilityDecision {
    /// La versión puede observarse en el timestamp solicitado.
    Visible,
    /// La lectura ocurrió antes de que la versión existiera.
    NotYetCreated {
        /// Timestamp de creación de la versión.
        created_at: LogicalTimestamp,
        /// Timestamp de lectura evaluado.
        read_at: LogicalTimestamp,
    },
    /// La lectura ocurrió después del cierre lógico de la versión.
    Deleted {
        /// Timestamp de cierre lógico de la versión.
        deleted_at: LogicalTimestamp,
        /// Timestamp de lectura evaluado.
        read_at: LogicalTimestamp,
    },
}

impl VisibilityDecision {
    /// Devuelve `true` cuando la decisión permite observar la versión.
    pub const fn is_visible(self) -> bool {
        matches!(self, Self::Visible)
    }
}

/// Versión concreta de un registro lógico.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordVersion {
    record_id: RecordId,
    version_id: VersionId,
    created_at: LogicalTimestamp,
    deleted_at: Option<LogicalTimestamp>,
    value: RecordValue,
}

impl RecordVersion {
    /// Crea una versión visible desde su timestamp de creación.
    pub fn new(
        record_id: RecordId,
        version_id: VersionId,
        created_at: LogicalTimestamp,
        value: RecordValue,
    ) -> Self {
        Self {
            record_id,
            version_id,
            created_at,
            deleted_at: None,
            value,
        }
    }

    /// Registro lógico al que pertenece la versión.
    pub fn record_id(&self) -> &RecordId {
        &self.record_id
    }

    /// Identificador de esta versión dentro de su cadena.
    pub fn version_id(&self) -> VersionId {
        self.version_id
    }

    /// Timestamp lógico en el que nació la versión.
    pub fn created_at(&self) -> LogicalTimestamp {
        self.created_at
    }

    /// Timestamp lógico que cerró la versión, si ya fue borrada.
    pub fn deleted_at(&self) -> Option<LogicalTimestamp> {
        self.deleted_at
    }

    /// Valor asociado a esta versión.
    pub fn value(&self) -> &RecordValue {
        &self.value
    }

    /// Devuelve `true` cuando la versión ya tiene cierre lógico.
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    /// Devuelve `true` cuando esta versión es visible para el snapshot.
    pub fn is_visible_in(&self, snapshot: &Snapshot) -> bool {
        self.is_visible_at(snapshot.read_at())
    }

    /// Devuelve `true` cuando esta versión es visible en un timestamp lógico.
    pub fn is_visible_at(&self, read_at: LogicalTimestamp) -> bool {
        self.visibility_at(read_at).is_visible()
    }

    /// Evalúa por qué una versión es visible o no en un timestamp lógico.
    pub fn visibility_at(&self, read_at: LogicalTimestamp) -> VisibilityDecision {
        if read_at < self.created_at {
            return VisibilityDecision::NotYetCreated {
                created_at: self.created_at,
                read_at,
            };
        }

        if let Some(deleted_at) = self.deleted_at {
            if deleted_at <= read_at {
                return VisibilityDecision::Deleted {
                    deleted_at,
                    read_at,
                };
            }
        }

        VisibilityDecision::Visible
    }

    /// Marca la versión como borrada en un timestamp lógico posterior o igual.
    pub fn delete_at(&mut self, deleted_at: LogicalTimestamp) -> Result<(), MvccError> {
        if self.deleted_at.is_some() {
            return Err(MvccError::VersionAlreadyDeleted {
                version_id: self.version_id,
            });
        }

        if deleted_at < self.created_at {
            return Err(MvccError::DeleteBeforeCreate {
                created_at: self.created_at,
                deleted_at,
            });
        }

        self.deleted_at = Some(deleted_at);
        Ok(())
    }
}

/// Errores del modelo educativo de MVCC.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MvccError {
    /// El identificador de registro está vacío.
    BlankRecordId,
    /// El valor de registro está vacío.
    BlankRecordValue,
    /// Una versión no puede borrarse antes de nacer.
    DeleteBeforeCreate {
        /// Timestamp de creación de la versión.
        created_at: LogicalTimestamp,
        /// Timestamp de borrado solicitado.
        deleted_at: LogicalTimestamp,
    },
    /// Una versión ya cerrada recibió otro borrado lógico.
    VersionAlreadyDeleted {
        /// Versión que ya estaba cerrada.
        version_id: VersionId,
    },
    /// La cadena recibió una versión con timestamp anterior a la última.
    NonMonotonicTimestamp {
        /// Último timestamp aceptado.
        previous: LogicalTimestamp,
        /// Timestamp solicitado para la nueva versión.
        next: LogicalTimestamp,
    },
    /// La cadena no tiene versiones que cerrar.
    EmptyVersionChain,
}

fn normalize(value: String) -> String {
    value.trim().to_owned()
}
