//! Modelos educativos mínimos de propiedades ACID.
//!
//! Este módulo no implementa un motor transaccional completo. Su propósito es
//! hacer visibles cuatro promesas: atomicidad como unidad de cierre,
//! consistencia como defensa de invariantes, aislamiento como lectura de valor
//! confirmado y durabilidad como commit sincronizado.

use std::collections::{BTreeMap, BTreeSet};

/// Propiedad ACID nombrada de forma estable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcidProperty {
    /// Una unidad de trabajo se acepta o descarta completa.
    Atomicity,
    /// Las invariantes se preservan entre estados válidos.
    Consistency,
    /// La concurrencia no expone trabajo incoherente.
    Isolation,
    /// Un commit confirmado puede recuperarse después de fallas cubiertas.
    Durability,
}

impl AcidProperty {
    /// Nombre estable de la propiedad.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Atomicity => "atomicity",
            Self::Consistency => "consistency",
            Self::Isolation => "isolation",
            Self::Durability => "durability",
        }
    }
}

/// Unidad mínima para enseñar atomicidad.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtomicUnit {
    state: UnitState,
    staged_changes: Vec<Change>,
    committed_changes: Vec<Change>,
}

impl AtomicUnit {
    /// Crea una unidad abierta sin cambios.
    pub fn new() -> Self {
        Self {
            state: UnitState::Active,
            staged_changes: Vec::new(),
            committed_changes: Vec::new(),
        }
    }

    /// Agrega un cambio tentativo.
    pub fn stage(&mut self, change: Change) -> Result<(), AcidError> {
        self.ensure_active()?;
        self.staged_changes.push(change);
        Ok(())
    }

    /// Confirma todos los cambios preparados como una unidad.
    pub fn commit(&mut self) -> Result<(), AcidError> {
        self.ensure_active()?;
        self.committed_changes.append(&mut self.staged_changes);
        self.state = UnitState::Committed;
        Ok(())
    }

    /// Descarta todos los cambios preparados como una unidad.
    pub fn rollback(&mut self) -> Result<(), AcidError> {
        self.ensure_active()?;
        self.staged_changes.clear();
        self.state = UnitState::RolledBack;
        Ok(())
    }

    /// Estado visible de la unidad.
    pub fn state(&self) -> UnitState {
        self.state
    }

    /// Cambios tentativos que todavía no se confirmaron.
    pub fn staged_changes(&self) -> &[Change] {
        &self.staged_changes
    }

    /// Cambios aceptados por `commit`.
    pub fn committed_changes(&self) -> &[Change] {
        &self.committed_changes
    }

    fn ensure_active(&self) -> Result<(), AcidError> {
        if self.state == UnitState::Active {
            return Ok(());
        }

        Err(AcidError::ClosedAtomicUnit { state: self.state })
    }
}

impl Default for AtomicUnit {
    fn default() -> Self {
        Self::new()
    }
}

/// Estado mínimo de una unidad atómica.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitState {
    /// Todavía acepta cambios tentativos.
    Active,
    /// Cerró aceptando todos sus cambios tentativos.
    Committed,
    /// Cerró descartando todos sus cambios tentativos.
    RolledBack,
}

/// Cambio lógico usado por el modelo de atomicidad.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Change(String);

impl Change {
    /// Devuelve la descripción estable del cambio.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Change {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

/// Constraint única para enseñar consistencia.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniqueConstraint {
    name: String,
    values: BTreeSet<String>,
}

impl UniqueConstraint {
    /// Crea una constraint de unicidad con nombre lógico.
    pub fn new(name: impl Into<String>) -> Result<Self, AcidError> {
        let name = normalize(name.into());

        if name.is_empty() {
            return Err(AcidError::BlankInvariantName);
        }

        Ok(Self {
            name,
            values: BTreeSet::new(),
        })
    }

    /// Inserta un valor si no rompe la unicidad.
    pub fn insert(&mut self, value: impl Into<String>) -> Result<(), AcidError> {
        let value = normalize(value.into());

        if value.is_empty() {
            return Err(AcidError::BlankInvariantValue);
        }

        if !self.values.insert(value.clone()) {
            return Err(AcidError::ConsistencyViolation {
                invariant: self.name.clone(),
                value,
            });
        }

        Ok(())
    }

    /// Número de valores aceptados.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Devuelve `true` cuando no hay valores registrados.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Revisa si un valor ya fue aceptado.
    pub fn contains(&self, value: &str) -> bool {
        self.values.contains(value)
    }

    /// Nombre lógico de la constraint.
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Identificador lógico de operación concurrente.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OperationId(u64);

impl OperationId {
    /// Crea un identificador lógico de operación.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico del identificador.
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// Celda educativa que enseña lectura confirmada.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadCommittedCell {
    committed: String,
    pending: Option<PendingWrite>,
}

impl ReadCommittedCell {
    /// Crea una celda con un valor confirmado inicial.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            committed: value.into(),
            pending: None,
        }
    }

    /// Prepara un valor sin hacerlo visible a lecturas confirmadas.
    pub fn write_pending(
        &mut self,
        writer: OperationId,
        value: impl Into<String>,
    ) -> Result<(), AcidError> {
        if let Some(pending) = &self.pending {
            if pending.writer != writer {
                return Err(AcidError::PendingWriteConflict {
                    holder: pending.writer,
                    requester: writer,
                });
            }
        }

        self.pending = Some(PendingWrite {
            writer,
            value: value.into(),
        });
        Ok(())
    }

    /// Lee solo el valor confirmado.
    pub fn read_committed(&self) -> &str {
        &self.committed
    }

    /// Escritor que mantiene un valor pendiente.
    pub fn pending_writer(&self) -> Option<OperationId> {
        self.pending.as_ref().map(|pending| pending.writer)
    }

    /// Publica el valor pendiente del escritor indicado.
    pub fn commit_pending(&mut self, writer: OperationId) -> Result<(), AcidError> {
        let pending = self.take_pending_for(writer)?;
        self.committed = pending.value;
        Ok(())
    }

    /// Descarta el valor pendiente del escritor indicado.
    pub fn rollback_pending(&mut self, writer: OperationId) -> Result<(), AcidError> {
        self.take_pending_for(writer)?;
        Ok(())
    }

    fn take_pending_for(&mut self, writer: OperationId) -> Result<PendingWrite, AcidError> {
        let pending = self.pending.take().ok_or(AcidError::NoPendingWrite)?;

        if pending.writer != writer {
            let holder = pending.writer;
            self.pending = Some(pending);
            return Err(AcidError::PendingWriteConflict {
                holder,
                requester: writer,
            });
        }

        Ok(pending)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PendingWrite {
    writer: OperationId,
    value: String,
}

/// Log educativo para representar commits durables.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitLog {
    next_commit_id: CommitId,
    records: BTreeMap<CommitId, CommitRecord>,
}

impl CommitLog {
    /// Crea un log sin commits.
    pub fn new() -> Self {
        Self {
            next_commit_id: CommitId::new(1),
            records: BTreeMap::new(),
        }
    }

    /// Agrega un commit todavía no sincronizado.
    pub fn append_commit(&mut self, label: impl Into<String>) -> Result<CommitId, AcidError> {
        let label = normalize(label.into());

        if label.is_empty() {
            return Err(AcidError::BlankCommitLabel);
        }

        let commit_id = self.next_commit_id;
        self.records.insert(
            commit_id,
            CommitRecord {
                label,
                synced: false,
            },
        );
        self.next_commit_id = commit_id.next();

        Ok(commit_id)
    }

    /// Marca un commit conocido como sincronizado.
    pub fn sync(&mut self, commit_id: CommitId) -> Result<(), AcidError> {
        let record = self
            .records
            .get_mut(&commit_id)
            .ok_or(AcidError::UnknownCommit(commit_id))?;

        record.synced = true;
        Ok(())
    }

    /// Devuelve si el commit ya es durable dentro de este modelo.
    pub fn is_durable(&self, commit_id: CommitId) -> Option<bool> {
        self.records.get(&commit_id).map(|record| record.synced)
    }

    /// Etiqueta registrada para un commit.
    pub fn label(&self, commit_id: CommitId) -> Option<&str> {
        self.records
            .get(&commit_id)
            .map(|record| record.label.as_str())
    }
}

impl Default for CommitLog {
    fn default() -> Self {
        Self::new()
    }
}

/// Identificador lógico de un commit en el log educativo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommitId(u64);

impl CommitId {
    /// Crea un identificador lógico de commit.
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct CommitRecord {
    label: String,
    synced: bool,
}

/// Errores de los modelos educativos ACID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AcidError {
    /// Una unidad atómica ya estaba cerrada.
    ClosedAtomicUnit {
        /// Estado terminal observado.
        state: UnitState,
    },
    /// Una constraint se creó sin nombre.
    BlankInvariantName,
    /// Se intentó validar un valor vacío.
    BlankInvariantValue,
    /// Una operación rompió una invariante de consistencia.
    ConsistencyViolation {
        /// Nombre lógico de la invariante.
        invariant: String,
        /// Valor que rompió la invariante.
        value: String,
    },
    /// Dos operaciones intentaron mantener escrituras pendientes incompatibles.
    PendingWriteConflict {
        /// Operación que conserva la escritura pendiente.
        holder: OperationId,
        /// Operación que intentó escribir.
        requester: OperationId,
    },
    /// No existe una escritura pendiente para confirmar o descartar.
    NoPendingWrite,
    /// Se intentó registrar un commit sin etiqueta.
    BlankCommitLabel,
    /// El commit buscado no existe en el log.
    UnknownCommit(CommitId),
}

fn normalize(value: String) -> String {
    value.trim().to_owned()
}
