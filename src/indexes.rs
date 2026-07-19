//! Modelo educativo inicial de índices.
//!
//! Este módulo separa el papel de un índice primario y un índice secundario
//! antes de modelar selectividad o mantenimiento. Un índice primario resuelve
//! una clave hacia la ubicación lógica del registro; un índice secundario
//! resuelve una clave alternativa hacia la clave primaria.

use std::collections::BTreeMap;

/// Definición declarativa de un índice educativo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexDefinition {
    name: IndexName,
    role: IndexRole,
    uniqueness: IndexUniqueness,
    key_columns: Vec<ColumnName>,
    target: IndexTarget,
}

impl IndexDefinition {
    /// Crea un índice primario sobre una columna.
    pub fn primary(name: IndexName, key_column: ColumnName) -> Self {
        Self {
            name,
            role: IndexRole::Primary,
            uniqueness: IndexUniqueness::Unique,
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
            uniqueness: IndexUniqueness::NonUnique,
            key_columns: vec![key_column],
            target: IndexTarget::PrimaryKey(primary_key_column),
        }
    }

    /// Crea un índice secundario único que apunta a la columna de primary key.
    pub fn unique_secondary(
        name: IndexName,
        key_column: ColumnName,
        primary_key_column: ColumnName,
    ) -> Self {
        Self {
            name,
            role: IndexRole::Secondary,
            uniqueness: IndexUniqueness::Unique,
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

    /// Regla de unicidad de las llaves dentro del índice.
    pub fn uniqueness(&self) -> IndexUniqueness {
        self.uniqueness
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

/// Regla educativa de unicidad para entradas de índice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexUniqueness {
    /// Cada llave de índice puede aparecer una sola vez.
    Unique,
    /// Una llave de índice puede apuntar a varias primary keys.
    NonUnique,
}

/// Destino lógico de una búsqueda en el índice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexTarget {
    /// La clave del índice apunta directamente a la ubicación del registro.
    RecordPointer,
    /// La clave del índice apunta a la columna que contiene la primary key.
    PrimaryKey(ColumnName),
}

/// Entradas educativas de un índice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexEntries {
    uniqueness: IndexUniqueness,
    entries: BTreeMap<IndexEntryKey, Vec<PrimaryKeyValue>>,
}

impl IndexEntries {
    /// Crea una colección vacía con la regla de unicidad indicada.
    pub fn new(uniqueness: IndexUniqueness) -> Self {
        Self {
            uniqueness,
            entries: BTreeMap::new(),
        }
    }

    /// Inserta una relación entre llave de índice y primary key.
    pub fn insert(
        &mut self,
        key: IndexEntryKey,
        primary_key: PrimaryKeyValue,
    ) -> Result<(), IndexError> {
        let primary_keys = self.entries.entry(key.clone()).or_default();

        if self.uniqueness == IndexUniqueness::Unique && !primary_keys.is_empty() {
            return Err(IndexError::DuplicateIndexKey(key));
        }

        primary_keys.push(primary_key);
        Ok(())
    }

    /// Devuelve las primary keys asociadas a una llave de índice.
    pub fn primary_keys_for(&self, key: &IndexEntryKey) -> Vec<PrimaryKeyValue> {
        self.entries.get(key).cloned().unwrap_or_default()
    }

    /// Calcula la selectividad como llaves distintas entre filas indexadas.
    pub fn selectivity(&self) -> Selectivity {
        let distinct_keys = self.entries.len();
        let indexed_rows = self.entries.values().map(std::vec::Vec::len).sum::<usize>();

        Selectivity::new(distinct_keys, indexed_rows)
    }

    /// Estima cuántas filas candidatas devolvería una búsqueda por llave.
    pub fn estimated_candidates_for(&self, key: &IndexEntryKey) -> usize {
        self.entries.get(key).map_or(0, std::vec::Vec::len)
    }
}

/// Selectividad educativa de un índice.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Selectivity {
    distinct_keys: usize,
    indexed_rows: usize,
}

impl Selectivity {
    /// Crea una métrica de selectividad.
    pub const fn new(distinct_keys: usize, indexed_rows: usize) -> Self {
        Self {
            distinct_keys,
            indexed_rows,
        }
    }

    /// Número de llaves distintas almacenadas en el índice.
    pub const fn distinct_keys(self) -> usize {
        self.distinct_keys
    }

    /// Número total de filas referenciadas por el índice.
    pub const fn indexed_rows(self) -> usize {
        self.indexed_rows
    }

    /// Proporción `distinct_keys / indexed_rows`.
    pub fn ratio(self) -> f64 {
        if self.indexed_rows == 0 {
            return 0.0;
        }

        self.distinct_keys as f64 / self.indexed_rows as f64
    }

    /// Clasifica la selectividad para razonar sobre costo de consulta.
    pub fn class(self) -> SelectivityClass {
        let ratio = self.ratio();

        if self.indexed_rows == 0 {
            SelectivityClass::Empty
        } else if ratio >= 0.8 {
            SelectivityClass::High
        } else if ratio >= 0.3 {
            SelectivityClass::Medium
        } else {
            SelectivityClass::Low
        }
    }
}

/// Clasificación educativa de selectividad.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectivityClass {
    /// El índice no contiene filas.
    Empty,
    /// Muchas llaves distintas para las filas indexadas.
    High,
    /// Algunas llaves se repiten, pero todavía reducen candidatos.
    Medium,
    /// Pocas llaves distintas para muchas filas.
    Low,
}

/// Llave almacenada dentro de un índice educativo.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexEntryKey(String);

impl IndexEntryKey {
    /// Devuelve el valor textual de la llave.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for IndexEntryKey {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

/// Valor de primary key referenciado por una entrada de índice.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrimaryKeyValue(String);

impl PrimaryKeyValue {
    /// Devuelve el valor textual de la primary key.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for PrimaryKeyValue {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
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
    /// Un índice único recibió una llave que ya existía.
    DuplicateIndexKey(IndexEntryKey),
    /// Una invariante interna fue violada.
    InvariantViolation(&'static str),
}
