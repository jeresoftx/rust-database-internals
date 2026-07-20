# Query Optimizer

> **Estado:** draft.
> **Alcance actual:** representación educativa de plan lógico y plan físico.
> Incluye table scan e index scan como alternativas de acceso físico. Todavía
> no modela estimación de costo ni relación con `EXPLAIN`.

## Por qué existe

Un motor de base de datos no ejecuta una consulta directamente desde el texto
SQL. Primero necesita representarla como una intención y después convertir esa
intención en una forma concreta de ejecución.

La separación importa porque dos consultas con la misma intención pueden
ejecutarse de maneras distintas:

```text
intención:
  dame id y balance de accounts donde status = active

posibles ejecuciones futuras:
  leer toda la tabla y filtrar
  usar un índice por status y luego proyectar columnas
```

Este primer paso del capítulo no decide cuál ejecución conviene. Solo crea el
vocabulario para distinguir:

- qué quiere la consulta;
- cómo podría ejecutarse;
- qué decisiones todavía no ha tomado el optimizador.

## Modelo mental

El plan lógico representa la intención:

```text
Project(id, balance)
  Select(status = active)
    ReadRelation(accounts)
```

El plan físico representa una forma de ejecución:

```text
Project(id)
  Filter(status = active)
    ReadRelation(accounts, access_path = table_scan)

Project(id)
  Filter(status = active)
    ReadRelation(accounts, access_path = index_scan(idx_accounts_status, status))
```

La diferencia parece pequeña, pero es fundamental. El plan lógico dice "quiero
estas columnas y este filtro". El plan físico empieza a hablar de ejecución:
operadores, orden de trabajo y ruta de acceso. En este punto ya puede nombrar
table scan e index scan, aunque todavía no decide cuál conviene.

## Modelo Rust actual

El módulo `src/query_optimizer.rs` expone nombres, predicados y dos árboles de
plan.

| Tipo | Responsabilidad |
|------|-----------------|
| `RelationName` | Nombre validado de una relación consultable. |
| `ColumnName` | Nombre validado de una columna. |
| `IndexName` | Nombre validado de un índice disponible. |
| `Literal` | Valor literal usado en predicados. |
| `ComparisonOperator` | Operador de comparación educativo. |
| `Predicate` | Comparación entre columna, operador y literal. |
| `LogicalPlan` | Árbol de intención de consulta. |
| `PhysicalPlan` | Árbol de forma de ejecución. |
| `PhysicalAccessPath` | Ruta de acceso física elegida o pendiente de elegir. |

`PhysicalAccessPath` reconoce tres estados:

- `Unchosen`: el optimizador todavía no elige una ruta;
- `TableScan`: leer toda la relación;
- `IndexScan`: leer mediante un índice nombrado y una columna de búsqueda.

## Invariantes

- un nombre de relación no puede estar vacío;
- un nombre de columna no puede estar vacío;
- un nombre de índice no puede estar vacío;
- una proyección debe pedir al menos una columna;
- un plan lógico `Select` o `Project` envuelve exactamente un hijo;
- un plan físico `Filter` o `Project` envuelve exactamente un hijo;
- `PhysicalAccessPath::Unchosen` significa que el optimizador aún no eligió
  table scan ni index scan;
- un index scan siempre nombra el índice usado y la columna de búsqueda.

## Diagrama

```mermaid
flowchart TD
    SQL["consulta"]
    L["plan lógico<br/>intención"]
    P["plan físico<br/>forma de ejecución"]
    A["access path<br/>unchosen"]
    T["table scan<br/>leer todo"]
    I["index scan<br/>leer por índice"]
    C["costo<br/>pendiente"]

    SQL --> L
    L --> P
    P --> A
    A --> T
    A --> I
    T --> C
    I --> C
```

## Ejemplo básico

```rust
use rust_database_internals::query_optimizer::{
    ColumnName, ComparisonOperator, Literal, LogicalPlan, Predicate, RelationName,
};

let plan = LogicalPlan::relation(RelationName::new("accounts")?)
    .select(Predicate::comparison(
        ColumnName::new("status")?,
        ComparisonOperator::Eq,
        Literal::text("active"),
    ))
    .project(vec![
        ColumnName::new("id")?,
        ColumnName::new("balance")?,
    ])?;

assert_eq!(plan.children().len(), 1);
# Ok::<(), rust_database_internals::query_optimizer::QueryOptimizerError>(())
```

## Table scan e index scan

Un table scan representa la opción más directa: leer toda la relación y dejar
que filtros posteriores descarten filas.

```rust
use rust_database_internals::query_optimizer::{
    PhysicalAccessPath, PhysicalOperation, PhysicalPlan, RelationName,
};

let relation = RelationName::new("accounts")?;
let plan = PhysicalPlan::table_scan(relation.clone());

assert_eq!(
    plan.operation(),
    &PhysicalOperation::ReadRelation {
        relation,
        access_path: PhysicalAccessPath::TableScan,
    }
);
# Ok::<(), rust_database_internals::query_optimizer::QueryOptimizerError>(())
```

Un index scan representa una ruta de acceso más específica: usar un índice
nombrado para buscar por una columna.

```rust
use rust_database_internals::query_optimizer::{
    ColumnName, IndexName, PhysicalAccessPath, PhysicalPlan, PhysicalOperation, RelationName,
};

let relation = RelationName::new("accounts")?;
let index = IndexName::new("idx_accounts_status")?;
let lookup_column = ColumnName::new("status")?;
let plan = PhysicalPlan::index_scan(relation.clone(), index.clone(), lookup_column.clone());

assert_eq!(
    plan.operation(),
    &PhysicalOperation::ReadRelation {
        relation,
        access_path: PhysicalAccessPath::IndexScan {
            index,
            lookup_column,
        },
    }
);
# Ok::<(), rust_database_internals::query_optimizer::QueryOptimizerError>(())
```

## Lo que aún no hace

Este borrador todavía no decide:

- cómo estimar costo;
- cómo elegir entre dos planes físicos;
- cómo explicar una decisión con una salida similar a `EXPLAIN`.

## Siguiente paso natural

El siguiente paso del capítulo es modelar estimación de costo para comparar
planes físicos.
