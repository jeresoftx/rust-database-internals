//! Representación educativa de planes para query optimizer.
//!
//! Este módulo separa dos preguntas:
//!
//! - plan lógico: qué quiere expresar la consulta;
//! - plan físico: con qué forma de ejecución podría realizarse.
//!
//! La elección entre table scan, index scan y costo queda para pasos
//! posteriores del capítulo.

use std::{error::Error, fmt};

/// Nombre lógico de una relación consultable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RelationName(String);

impl RelationName {
    /// Crea un nombre de relación normalizado.
    pub fn new(value: impl Into<String>) -> Result<Self, QueryOptimizerError> {
        let value = normalize(value.into());

        if value.is_empty() {
            return Err(QueryOptimizerError::BlankRelationName);
        }

        Ok(Self(value))
    }

    /// Devuelve el nombre como texto.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Nombre lógico de una columna.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColumnName(String);

impl ColumnName {
    /// Crea un nombre de columna normalizado.
    pub fn new(value: impl Into<String>) -> Result<Self, QueryOptimizerError> {
        let value = normalize(value.into());

        if value.is_empty() {
            return Err(QueryOptimizerError::BlankColumnName);
        }

        Ok(Self(value))
    }

    /// Devuelve el nombre como texto.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Valor literal usado por un predicado educativo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    /// Texto.
    Text(String),
    /// Entero firmado.
    Integer(i64),
    /// Booleano.
    Boolean(bool),
}

impl Literal {
    /// Crea un literal de texto.
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text(value.into())
    }
}

/// Operador de comparación de un predicado.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonOperator {
    /// Igualdad.
    Eq,
    /// Diferencia.
    NotEq,
    /// Menor que.
    Lt,
    /// Menor o igual que.
    Lte,
    /// Mayor que.
    Gt,
    /// Mayor o igual que.
    Gte,
}

/// Predicado lógico de una consulta.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Predicate {
    column: ColumnName,
    operator: ComparisonOperator,
    value: Literal,
}

impl Predicate {
    /// Crea un predicado de comparación sobre una columna.
    pub const fn comparison(
        column: ColumnName,
        operator: ComparisonOperator,
        value: Literal,
    ) -> Self {
        Self {
            column,
            operator,
            value,
        }
    }

    /// Columna comparada.
    pub const fn column(&self) -> &ColumnName {
        &self.column
    }

    /// Operador de comparación.
    pub const fn operator(&self) -> ComparisonOperator {
        self.operator
    }

    /// Valor literal comparado.
    pub const fn value(&self) -> &Literal {
        &self.value
    }
}

/// Operación dentro de un plan lógico.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicalOperation {
    /// Lee una relación sin decidir todavía cómo acceder físicamente a ella.
    ReadRelation {
        /// Relación consultada.
        relation: RelationName,
    },
    /// Filtra filas por predicado.
    Select {
        /// Predicado lógico.
        predicate: Predicate,
    },
    /// Proyecta columnas.
    Project {
        /// Columnas solicitadas.
        columns: Vec<ColumnName>,
    },
}

/// Árbol lógico de consulta.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalPlan {
    operation: LogicalOperation,
    children: Vec<LogicalPlan>,
}

impl LogicalPlan {
    /// Crea una hoja lógica para leer una relación.
    pub fn relation(relation: RelationName) -> Self {
        Self {
            operation: LogicalOperation::ReadRelation { relation },
            children: vec![],
        }
    }

    /// Agrega una selección encima del plan actual.
    pub fn select(self, predicate: Predicate) -> Self {
        Self {
            operation: LogicalOperation::Select { predicate },
            children: vec![self],
        }
    }

    /// Agrega una proyección encima del plan actual.
    pub fn project(self, columns: Vec<ColumnName>) -> Result<Self, QueryOptimizerError> {
        if columns.is_empty() {
            return Err(QueryOptimizerError::ProjectionRequiresColumns);
        }

        Ok(Self {
            operation: LogicalOperation::Project { columns },
            children: vec![self],
        })
    }

    /// Operación de este nodo.
    pub const fn operation(&self) -> &LogicalOperation {
        &self.operation
    }

    /// Hijos del nodo.
    pub fn children(&self) -> &[LogicalPlan] {
        &self.children
    }
}

/// Ruta de acceso físico elegida o pendiente de elegir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalAccessPath {
    /// El optimizador todavía no eligió table scan ni index scan.
    Unchosen,
}

/// Operación dentro de un plan físico.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalOperation {
    /// Lee una relación usando una ruta de acceso física.
    ReadRelation {
        /// Relación consultada.
        relation: RelationName,
        /// Ruta de acceso física.
        access_path: PhysicalAccessPath,
    },
    /// Aplica un filtro durante ejecución.
    Filter {
        /// Predicado físico.
        predicate: Predicate,
    },
    /// Produce solo ciertas columnas.
    Project {
        /// Columnas solicitadas.
        columns: Vec<ColumnName>,
    },
}

/// Árbol físico de ejecución.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalPlan {
    operation: PhysicalOperation,
    children: Vec<PhysicalPlan>,
}

impl PhysicalPlan {
    /// Crea una hoja física de lectura de relación.
    pub fn read_relation(relation: RelationName, access_path: PhysicalAccessPath) -> Self {
        Self {
            operation: PhysicalOperation::ReadRelation {
                relation,
                access_path,
            },
            children: vec![],
        }
    }

    /// Agrega un filtro encima del plan actual.
    pub fn filter(self, predicate: Predicate) -> Self {
        Self {
            operation: PhysicalOperation::Filter { predicate },
            children: vec![self],
        }
    }

    /// Agrega una proyección encima del plan actual.
    pub fn project(self, columns: Vec<ColumnName>) -> Result<Self, QueryOptimizerError> {
        if columns.is_empty() {
            return Err(QueryOptimizerError::ProjectionRequiresColumns);
        }

        Ok(Self {
            operation: PhysicalOperation::Project { columns },
            children: vec![self],
        })
    }

    /// Operación de este nodo.
    pub const fn operation(&self) -> &PhysicalOperation {
        &self.operation
    }

    /// Hijos del nodo.
    pub fn children(&self) -> &[PhysicalPlan] {
        &self.children
    }
}

/// Errores de representación del optimizador educativo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryOptimizerError {
    /// El nombre de relación está vacío.
    BlankRelationName,
    /// El nombre de columna está vacío.
    BlankColumnName,
    /// Una proyección debe pedir al menos una columna.
    ProjectionRequiresColumns,
}

impl fmt::Display for QueryOptimizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BlankRelationName => write!(f, "el nombre de relación no puede estar vacío"),
            Self::BlankColumnName => write!(f, "el nombre de columna no puede estar vacío"),
            Self::ProjectionRequiresColumns => {
                write!(f, "una proyección requiere al menos una columna")
            }
        }
    }
}

impl Error for QueryOptimizerError {}

fn normalize(value: String) -> String {
    value.trim().to_owned()
}
