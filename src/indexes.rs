//! Modelo educativo inicial de índices.
//!
//! Este módulo separa el papel de un índice primario y un índice secundario
//! antes de modelar unicidad, selectividad o mantenimiento. Un índice primario
//! resuelve una clave hacia la ubicación lógica del registro; un índice
//! secundario resuelve una clave alternativa hacia la clave primaria.

/// Definición declarativa de un índice educativo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexDefinition {
    name: IndexName,
    role: IndexRole,
    key_columns: Vec<ColumnName>,
    target: IndexTarget,
}

impl IndexDefinition {
    /// Crea un índice primario sobre una columna.
    pub fn primary(name: IndexName, key_column: ColumnName) -> Self {
        Self {
            name,
            role: IndexRole::Primary,
            key_columns: vec![key_column],
            target: IndexTarget::RecordPointer,
        }
    }

    /// Crea un índice secundario que apunta a la columna de primary key.
    pub fn secondary(
        name: IndexName,
        key_column: ColumnName,
        primary_key_column: ColumnName,
    ) -> Self {
        Self {
            name,
            role: IndexRole::Secondary,
            key_columns: vec![key_column],
            target: IndexTarget::PrimaryKey(primary_key_column),
        }
    }

    /// Nombre lógico del índice.
    pub fn name(&self) -> &IndexName {
        &self.name
    }

    /// Papel del índice dentro de la tabla.
    pub fn role(&self) -> IndexRole {
        self.role
    }

    /// Columnas que forman la llave de búsqueda del índice.
    pub fn key_columns(&self) -> &[ColumnName] {
        &self.key_columns
    }

    /// Destino al que resuelve una búsqueda en el índice.
    pub fn target(&self) -> &IndexTarget {
        &self.target
    }
}

/// Papel lógico de un índice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexRole {
    /// Índice que define la identidad principal de una fila.
    Primary,
    /// Índice alternativo que necesita volver a la primary key.
    Secondary,
}

/// Destino lógico de una búsqueda en el índice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexTarget {
    /// La clave del índice apunta directamente a la ubicación del registro.
    RecordPointer,
    /// La clave del índice apunta a la columna que contiene la primary key.
    PrimaryKey(ColumnName),
}

/// Nombre lógico de índice.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexName(String);

impl IndexName {
    /// Crea un nombre de índice no vacío.
    pub fn new(value: impl Into<String>) -> Result<Self, IndexError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(IndexError::BlankIndexName);
        }

        Ok(Self(value))
    }

    /// Devuelve el nombre como texto.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Nombre lógico de columna.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumnName(String);

impl ColumnName {
    /// Crea un nombre de columna no vacío.
    pub fn new(value: impl Into<String>) -> Result<Self, IndexError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(IndexError::BlankColumnName);
        }

        Ok(Self(value))
    }

    /// Devuelve el nombre como texto.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Errores del modelo educativo de índices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexError {
    /// El nombre del índice no puede estar vacío.
    BlankIndexName,
    /// El nombre de columna no puede estar vacío.
    BlankColumnName,
    /// Una invariante interna fue violada.
    InvariantViolation(&'static str),
}
