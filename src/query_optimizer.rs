//! Representación educativa de planes para query optimizer.
//!
//! Este módulo separa dos preguntas:
//!
//! - plan lógico: qué quiere expresar la consulta;
//! - plan físico: con qué forma de ejecución podría realizarse.
//!
//! La relación con `EXPLAIN`, ejemplos finales y benchmark quedan para pasos
//! posteriores del capítulo.

use std::{error::Error, fmt};

const INDEX_PROBE_WORK_UNITS: u64 = 10;

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

/// Nombre lógico de un índice disponible para ejecutar una consulta.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndexName(String);

impl IndexName {
    /// Crea un nombre de índice normalizado.
    pub fn new(value: impl Into<String>) -> Result<Self, QueryOptimizerError> {
        let value = normalize(value.into());

        if value.is_empty() {
            return Err(QueryOptimizerError::BlankIndexName);
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
    /// Leer todos los registros de una relación y filtrar después.
    TableScan,
    /// Leer mediante un índice nombrado sobre una columna de búsqueda.
    IndexScan {
        /// Índice usado para acceder a la relación.
        index: IndexName,
        /// Columna usada como llave de búsqueda en el índice.
        lookup_column: ColumnName,
    },
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

    /// Crea una hoja física que lee toda la relación.
    pub fn table_scan(relation: RelationName) -> Self {
        Self::read_relation(relation, PhysicalAccessPath::TableScan)
    }

    /// Crea una hoja física que accede por un índice nombrado.
    pub fn index_scan(relation: RelationName, index: IndexName, lookup_column: ColumnName) -> Self {
        Self::read_relation(
            relation,
            PhysicalAccessPath::IndexScan {
                index,
                lookup_column,
            },
        )
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

    /// Estima el costo educativo del plan físico.
    pub fn estimate_cost(&self, catalog: &CostCatalog) -> Result<PlanCost, QueryOptimizerError> {
        match &self.operation {
            PhysicalOperation::ReadRelation {
                relation,
                access_path,
            } => estimate_access_path_cost(relation, access_path, catalog),
            PhysicalOperation::Filter { .. } | PhysicalOperation::Project { .. } => self
                .children
                .first()
                .ok_or(QueryOptimizerError::PlanNodeRequiresChild)?
                .estimate_cost(catalog),
        }
    }
}

/// Conteo de filas conocido o estimado para una relación.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RowCount(u64);

impl RowCount {
    /// Crea un conteo de filas.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Devuelve el conteo como entero.
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Selectividad de un índice en puntos base.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selectivity(u16);

impl Selectivity {
    /// Crea una selectividad entre 0 y 10_000 puntos base.
    pub const fn new_basis_points(value: u16) -> Result<Self, QueryOptimizerError> {
        if value > 10_000 {
            return Err(QueryOptimizerError::InvalidSelectivity);
        }

        Ok(Self(value))
    }

    /// Devuelve los puntos base.
    pub const fn basis_points(self) -> u16 {
        self.0
    }

    fn estimate_rows(self, row_count: RowCount) -> u64 {
        let rows = row_count.get();
        let basis_points = u64::from(self.0);

        if rows == 0 || basis_points == 0 {
            return 0;
        }

        rows.saturating_mul(basis_points).div_ceil(10_000)
    }
}

/// Estadísticas educativas de una relación.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationStatistics {
    relation: RelationName,
    row_count: RowCount,
}

impl RelationStatistics {
    /// Crea estadísticas para una relación.
    pub const fn new(relation: RelationName, row_count: RowCount) -> Self {
        Self {
            relation,
            row_count,
        }
    }

    /// Relación descrita.
    pub const fn relation(&self) -> &RelationName {
        &self.relation
    }

    /// Filas estimadas.
    pub const fn row_count(&self) -> RowCount {
        self.row_count
    }
}

/// Estadísticas educativas de un índice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexStatistics {
    index: IndexName,
    selectivity: Selectivity,
}

impl IndexStatistics {
    /// Crea estadísticas para un índice.
    pub const fn new(index: IndexName, selectivity: Selectivity) -> Self {
        Self { index, selectivity }
    }

    /// Índice descrito.
    pub const fn index(&self) -> &IndexName {
        &self.index
    }

    /// Selectividad estimada.
    pub const fn selectivity(&self) -> Selectivity {
        self.selectivity
    }
}

/// Catálogo mínimo de estadísticas para estimar costos.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CostCatalog {
    relations: Vec<RelationStatistics>,
    indexes: Vec<IndexStatistics>,
}

impl CostCatalog {
    /// Crea un catálogo con estadísticas de relaciones.
    pub fn new(relations: Vec<RelationStatistics>) -> Self {
        Self {
            relations,
            indexes: vec![],
        }
    }

    /// Agrega estadísticas de índices al catálogo.
    pub fn with_indexes(mut self, indexes: Vec<IndexStatistics>) -> Self {
        self.indexes = indexes;
        self
    }

    /// Busca estadísticas de relación.
    pub fn relation(&self, relation: &RelationName) -> Option<&RelationStatistics> {
        self.relations
            .iter()
            .find(|statistics| statistics.relation() == relation)
    }

    /// Busca estadísticas de índice.
    pub fn index(&self, index: &IndexName) -> Option<&IndexStatistics> {
        self.indexes
            .iter()
            .find(|statistics| statistics.index() == index)
    }
}

/// Costo educativo estimado para un plan físico.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlanCost {
    rows_read: u64,
    rows_output: u64,
    work_units: u64,
}

impl PlanCost {
    /// Crea un costo estimado.
    pub const fn new(rows_read: u64, rows_output: u64, work_units: u64) -> Self {
        Self {
            rows_read,
            rows_output,
            work_units,
        }
    }

    /// Filas que el plan espera leer.
    pub const fn rows_read(self) -> u64 {
        self.rows_read
    }

    /// Filas que el plan espera producir.
    pub const fn rows_output(self) -> u64 {
        self.rows_output
    }

    /// Unidad abstracta de trabajo estimada.
    pub const fn work_units(self) -> u64 {
        self.work_units
    }

    /// Compara dos costos por trabajo estimado.
    pub const fn is_cheaper_than(&self, other: &Self) -> bool {
        self.work_units < other.work_units
    }
}

/// Errores de representación del optimizador educativo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryOptimizerError {
    /// El nombre de relación está vacío.
    BlankRelationName,
    /// El nombre de columna está vacío.
    BlankColumnName,
    /// El nombre de índice está vacío.
    BlankIndexName,
    /// Una proyección debe pedir al menos una columna.
    ProjectionRequiresColumns,
    /// La selectividad debe estar entre 0 y 10_000 puntos base.
    InvalidSelectivity,
    /// Faltan estadísticas de la relación.
    MissingRelationStatistics {
        /// Relación sin estadísticas.
        relation: RelationName,
    },
    /// Faltan estadísticas del índice.
    MissingIndexStatistics {
        /// Índice sin estadísticas.
        index: IndexName,
    },
    /// No se puede estimar una ruta de acceso todavía no elegida.
    CannotEstimateUnchosenAccessPath,
    /// Un nodo físico calculable necesita un hijo.
    PlanNodeRequiresChild,
}

impl fmt::Display for QueryOptimizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BlankRelationName => write!(f, "el nombre de relación no puede estar vacío"),
            Self::BlankColumnName => write!(f, "el nombre de columna no puede estar vacío"),
            Self::BlankIndexName => write!(f, "el nombre de índice no puede estar vacío"),
            Self::ProjectionRequiresColumns => {
                write!(f, "una proyección requiere al menos una columna")
            }
            Self::InvalidSelectivity => {
                write!(f, "la selectividad debe estar entre 0 y 10_000 puntos base")
            }
            Self::MissingRelationStatistics { relation } => write!(
                f,
                "faltan estadísticas de relación para '{}'",
                relation.as_str()
            ),
            Self::MissingIndexStatistics { index } => {
                write!(f, "faltan estadísticas de índice para '{}'", index.as_str())
            }
            Self::CannotEstimateUnchosenAccessPath => {
                write!(f, "no se puede estimar una ruta de acceso sin elegir")
            }
            Self::PlanNodeRequiresChild => write!(f, "el nodo físico necesita un hijo"),
        }
    }
}

impl Error for QueryOptimizerError {}

fn normalize(value: String) -> String {
    value.trim().to_owned()
}

fn estimate_access_path_cost(
    relation: &RelationName,
    access_path: &PhysicalAccessPath,
    catalog: &CostCatalog,
) -> Result<PlanCost, QueryOptimizerError> {
    let relation_statistics = catalog.relation(relation).ok_or_else(|| {
        QueryOptimizerError::MissingRelationStatistics {
            relation: relation.clone(),
        }
    })?;
    let row_count = relation_statistics.row_count();

    match access_path {
        PhysicalAccessPath::Unchosen => Err(QueryOptimizerError::CannotEstimateUnchosenAccessPath),
        PhysicalAccessPath::TableScan => {
            let rows = row_count.get();
            Ok(PlanCost::new(rows, rows, rows))
        }
        PhysicalAccessPath::IndexScan { index, .. } => {
            let index_statistics = catalog.index(index).ok_or_else(|| {
                QueryOptimizerError::MissingIndexStatistics {
                    index: index.clone(),
                }
            })?;
            let estimated_rows = index_statistics.selectivity().estimate_rows(row_count);
            let work_units = INDEX_PROBE_WORK_UNITS.saturating_add(estimated_rows);

            Ok(PlanCost::new(estimated_rows, estimated_rows, work_units))
        }
    }
}
