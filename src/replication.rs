//! Modelo educativo de replicación primary/replica.
//!
//! Este módulo inicia Replicación con el contrato más pequeño: solo el primary
//! acepta escrituras locales y una réplica recibe copias ordenadas del WAL.

use crate::wal::{LogOperation, LogSequenceNumber, WalError, WalTransactionId, WriteAheadLog};

/// Rol de un nodo dentro del modelo educativo de replicación.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicationRole {
    /// Nodo que acepta escrituras locales.
    Primary,
    /// Nodo que recibe cambios desde el primary.
    Replica,
}

/// Nodo de replicación con un log WAL local.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplicationNode {
    id: String,
    role: ReplicationRole,
    log: WriteAheadLog,
}

impl ReplicationNode {
    /// Crea un nodo primary.
    pub fn primary(id: impl Into<String>) -> Result<Self, ReplicationError> {
        Self::new(id, ReplicationRole::Primary)
    }

    /// Crea un nodo réplica.
    pub fn replica(id: impl Into<String>) -> Result<Self, ReplicationError> {
        Self::new(id, ReplicationRole::Replica)
    }

    fn new(id: impl Into<String>, role: ReplicationRole) -> Result<Self, ReplicationError> {
        let id = normalize(id.into());

        if id.is_empty() {
            return Err(ReplicationError::BlankNodeId);
        }

        Ok(Self {
            id,
            role,
            log: WriteAheadLog::new(),
        })
    }

    /// Identificador estable del nodo.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Rol del nodo.
    pub const fn role(&self) -> ReplicationRole {
        self.role
    }

    /// WAL local del nodo.
    pub const fn log(&self) -> &WriteAheadLog {
        &self.log
    }

    /// Agrega una actualización local. Solo el primary puede hacerlo.
    pub fn append_local_update(
        &mut self,
        transaction_id: WalTransactionId,
        operation: LogOperation,
    ) -> Result<(), ReplicationError> {
        self.ensure_primary()?;
        self.log.append(transaction_id, operation);
        Ok(())
    }

    /// Agrega un commit local. Solo el primary puede hacerlo.
    pub fn append_local_commit(
        &mut self,
        transaction_id: WalTransactionId,
    ) -> Result<(), ReplicationError> {
        self.ensure_primary()?;
        self.log.append_commit(transaction_id);
        Ok(())
    }

    fn ensure_primary(&self) -> Result<(), ReplicationError> {
        if self.role == ReplicationRole::Primary {
            Ok(())
        } else {
            Err(ReplicationError::ReplicaCannotAcceptLocalWrite {
                node_id: self.id.clone(),
            })
        }
    }

    fn copy_records_from(&mut self, primary: &ReplicationNode) -> Result<usize, ReplicationError> {
        let mut copied = 0;

        for record in primary.log.records().iter().skip(self.log.len()) {
            self.log.append_record(record.clone())?;
            copied += 1;
        }

        Ok(copied)
    }
}

/// Cluster educativo con un primary y cero o más réplicas.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplicationCluster {
    primary: ReplicationNode,
    replicas: Vec<ReplicationNode>,
}

impl ReplicationCluster {
    /// Crea un cluster validando roles y nombres únicos.
    pub fn new(
        primary: ReplicationNode,
        replicas: Vec<ReplicationNode>,
    ) -> Result<Self, ReplicationError> {
        if primary.role != ReplicationRole::Primary {
            return Err(ReplicationError::PrimaryRoleRequired {
                node_id: primary.id,
            });
        }

        let mut seen_ids = vec![primary.id.clone()];
        for replica in &replicas {
            if replica.role != ReplicationRole::Replica {
                return Err(ReplicationError::ReplicaRoleRequired {
                    node_id: replica.id.clone(),
                });
            }

            if seen_ids.iter().any(|seen| seen == &replica.id) {
                return Err(ReplicationError::DuplicateNodeId {
                    node_id: replica.id.clone(),
                });
            }

            seen_ids.push(replica.id.clone());
        }

        Ok(Self { primary, replicas })
    }

    /// Nodo primary del cluster.
    pub const fn primary(&self) -> &ReplicationNode {
        &self.primary
    }

    /// Réplicas conocidas.
    pub fn replicas(&self) -> &[ReplicationNode] {
        &self.replicas
    }

    /// Busca una réplica por identificador.
    pub fn replica(&self, id: &str) -> Option<&ReplicationNode> {
        self.replicas.iter().find(|replica| replica.id() == id)
    }

    /// Mide cuántos registros del primary aún no tiene una réplica.
    pub fn replica_lag(&self, replica_id: &str) -> Result<ReplicationLag, ReplicationError> {
        let replica = self
            .replica(replica_id)
            .ok_or_else(|| ReplicationError::UnknownReplica {
                node_id: replica_id.to_owned(),
            })?;
        let primary_records = self.primary.log.len();
        let replica_records = replica.log.len();

        Ok(ReplicationLag {
            primary_records,
            replica_records,
            primary_last_lsn: self.primary.log.last_lsn(),
            replica_last_lsn: replica.log.last_lsn(),
        })
    }

    /// Decide si una escritura puede considerarse confirmada.
    pub fn confirm_write(
        &self,
        mode: ReplicationAckMode,
    ) -> Result<ReplicationDecision, ReplicationError> {
        match mode {
            ReplicationAckMode::Async => Ok(ReplicationDecision::Confirmed),
            ReplicationAckMode::Sync => {
                let mut pending_replicas = 0;
                let mut pending_records = 0;

                for replica in &self.replicas {
                    let lag = self.replica_lag(replica.id())?;
                    if !lag.is_caught_up() {
                        pending_replicas += 1;
                        pending_records += lag.pending_records();
                    }
                }

                if pending_replicas == 0 {
                    Ok(ReplicationDecision::Confirmed)
                } else {
                    Ok(ReplicationDecision::WaitingForReplicas {
                        pending_replicas,
                        pending_records,
                    })
                }
            }
        }
    }

    /// Copia registros pendientes del primary hacia una réplica.
    pub fn replicate_to(
        &mut self,
        replica_id: &str,
    ) -> Result<ReplicationReport, ReplicationError> {
        let replica = self
            .replicas
            .iter_mut()
            .find(|replica| replica.id() == replica_id)
            .ok_or_else(|| ReplicationError::UnknownReplica {
                node_id: replica_id.to_owned(),
            })?;
        let copied_records = replica.copy_records_from(&self.primary)?;

        Ok(ReplicationReport { copied_records })
    }
}

/// Modo de confirmación de una escritura replicada.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicationAckMode {
    /// Confirmar cuando el primary aceptó la escritura localmente.
    Async,
    /// Confirmar solo cuando las réplicas conocidas alcanzaron al primary.
    Sync,
}

/// Decisión educativa de confirmación.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicationDecision {
    /// La escritura puede considerarse confirmada.
    Confirmed,
    /// La escritura espera a que una o más réplicas alcancen al primary.
    WaitingForReplicas {
        /// Réplicas con registros pendientes.
        pending_replicas: usize,
        /// Total de registros pendientes entre esas réplicas.
        pending_records: usize,
    },
}

/// Atraso observable de una réplica respecto al primary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplicationLag {
    primary_records: usize,
    replica_records: usize,
    primary_last_lsn: Option<LogSequenceNumber>,
    replica_last_lsn: Option<LogSequenceNumber>,
}

impl ReplicationLag {
    /// Registros conocidos por el primary.
    pub const fn primary_records(self) -> usize {
        self.primary_records
    }

    /// Registros conocidos por la réplica.
    pub const fn replica_records(self) -> usize {
        self.replica_records
    }

    /// Registros pendientes de copiar hacia la réplica.
    pub const fn pending_records(self) -> usize {
        self.primary_records.saturating_sub(self.replica_records)
    }

    /// Último LSN conocido por el primary.
    pub const fn primary_last_lsn(self) -> Option<LogSequenceNumber> {
        self.primary_last_lsn
    }

    /// Último LSN conocido por la réplica.
    pub const fn replica_last_lsn(self) -> Option<LogSequenceNumber> {
        self.replica_last_lsn
    }

    /// Devuelve `true` cuando la réplica ya copió todo el WAL conocido.
    pub const fn is_caught_up(self) -> bool {
        self.pending_records() == 0
    }
}

/// Resultado de copiar registros del primary hacia una réplica.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplicationReport {
    copied_records: usize,
}

impl ReplicationReport {
    /// Cantidad de registros copiados.
    pub const fn copied_records(self) -> usize {
        self.copied_records
    }
}

/// Errores del modelo educativo de replicación.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplicationError {
    /// El identificador del nodo está vacío.
    BlankNodeId,
    /// Un nodo réplica recibió una escritura local.
    ReplicaCannotAcceptLocalWrite {
        /// Nodo que rechazó la escritura.
        node_id: String,
    },
    /// El cluster requiere que el nodo principal tenga rol primary.
    PrimaryRoleRequired {
        /// Nodo inválido.
        node_id: String,
    },
    /// La lista de réplicas solo acepta nodos con rol replica.
    ReplicaRoleRequired {
        /// Nodo inválido.
        node_id: String,
    },
    /// No existe una réplica con ese identificador.
    UnknownReplica {
        /// Identificador solicitado.
        node_id: String,
    },
    /// Dos nodos comparten identificador.
    DuplicateNodeId {
        /// Identificador duplicado.
        node_id: String,
    },
    /// La copia de WAL encontró un error de orden.
    Wal(WalError),
}

impl From<WalError> for ReplicationError {
    fn from(error: WalError) -> Self {
        Self::Wal(error)
    }
}

fn normalize(value: String) -> String {
    value.trim().to_owned()
}
