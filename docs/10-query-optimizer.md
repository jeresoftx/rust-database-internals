# Query Optimizer

> **Estado:** benchmarked.
> **Alcance actual:** representación educativa de plan lógico y plan físico.
> Incluye table scan, index scan, estimación de costo, relación conceptual con
> `EXPLAIN`, ejemplos, ejercicios y benchmark manual.

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
table scan e index scan, y compararlos con un costo estimado.

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
| `CostCatalog` | Estadísticas mínimas de relaciones e índices. |
| `RelationStatistics` | Conteo de filas de una relación. |
| `IndexStatistics` | Selectividad de un índice. |
| `PlanCost` | Filas leídas, filas producidas y unidades de trabajo. |

`PhysicalAccessPath` reconoce tres estados:

- `Unchosen`: el optimizador todavía no elige una ruta;
- `TableScan`: leer toda la relación;
- `IndexScan`: leer mediante un índice nombrado y una columna de búsqueda.

La estimación de costo usa reglas deliberadamente pequeñas:

- table scan lee todas las filas de la relación;
- index scan lee las filas estimadas por selectividad;
- index scan suma un costo fijo de búsqueda de índice;
- el plan más barato es el de menor número de unidades de trabajo.

## Invariantes

- un nombre de relación no puede estar vacío;
- un nombre de columna no puede estar vacío;
- un nombre de índice no puede estar vacío;
- una proyección debe pedir al menos una columna;
- un plan lógico `Select` o `Project` envuelve exactamente un hijo;
- un plan físico `Filter` o `Project` envuelve exactamente un hijo;
- `PhysicalAccessPath::Unchosen` significa que el optimizador aún no eligió
  table scan ni index scan;
- un index scan siempre nombra el índice usado y la columna de búsqueda;
- la selectividad de un índice debe estar entre 0 y 10_000 puntos base;
- una ruta `Unchosen` no puede estimarse hasta elegir una alternativa física.

## Diagrama

```mermaid
flowchart TD
    SQL["consulta"]
    L["plan lógico<br/>intención"]
    P["plan físico<br/>forma de ejecución"]
    A["access path<br/>unchosen"]
    T["table scan<br/>leer todo"]
    I["index scan<br/>leer por índice"]
    C["costo<br/>filas y trabajo"]
    B["comparar<br/>menor costo"]

    SQL --> L
    L --> P
    P --> A
    A --> T
    A --> I
    T --> C
    I --> C
    C --> B
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

## Estimación de costo

La estimación no intenta predecir un motor real. Sirve para practicar la idea
central: un optimizador compara alternativas usando estadísticas.

```rust
use rust_database_internals::query_optimizer::{
    ColumnName, CostCatalog, IndexName, IndexStatistics, PhysicalPlan, RelationName,
    RelationStatistics, RowCount, Selectivity,
};

let relation = RelationName::new("accounts")?;
let index = IndexName::new("idx_accounts_status")?;
let catalog = CostCatalog::new(vec![RelationStatistics::new(
    relation.clone(),
    RowCount::new(10_000),
)])
.with_indexes(vec![IndexStatistics::new(
    index.clone(),
    Selectivity::new_basis_points(500)?,
)]);

let table_scan = PhysicalPlan::table_scan(relation.clone());
let index_scan = PhysicalPlan::index_scan(
    relation,
    index,
    ColumnName::new("status")?,
);

let table_cost = table_scan.estimate_cost(&catalog)?;
let index_cost = index_scan.estimate_cost(&catalog)?;

assert_eq!(table_cost.work_units(), 10_000);
assert_eq!(index_cost.work_units(), 510);
assert!(index_cost.is_cheaper_than(&table_cost));
# Ok::<(), rust_database_internals::query_optimizer::QueryOptimizerError>(())
```

## Por qué EXPLAIN existe

Un motor real puede elegir un plan que no coincide con la intuición de quien
escribió la consulta. `EXPLAIN` existe para abrir esa decisión.

Sin una herramienta así, una consulta lenta queda como misterio:

```text
SELECT id, balance
FROM accounts
WHERE status = 'active';
```

El texto SQL no dice si el motor leyó toda la tabla, usó un índice, cuántas
filas esperaba leer ni qué costo estimó. `EXPLAIN` muestra el plan que el motor
decidió usar.

En este capítulo, una salida conceptual puede verse así:

```text
Project(id, balance)
  Filter(status = active)
    IndexScan(accounts using idx_accounts_status)

estimated rows read: 500
estimated rows output: 500
estimated work units: 510
```

La palabra importante es "estimated". Un optimizador no conoce el futuro:
trabaja con estadísticas. Si las estadísticas están atrasadas, si la
selectividad cambia o si la distribución real de datos está sesgada, el plan
elegido puede ser peor de lo esperado.

Por eso `EXPLAIN` no es solo una herramienta para "ver si usa índice". Es una
forma de leer la hipótesis del motor:

- qué relación lee;
- qué ruta de acceso usa;
- qué filtros aplica;
- cuántas filas cree que leerá;
- cuánto trabajo estima;
- dónde podría estar equivocada su suposición.

Este modelo educativo no implementa SQL ni reproduce la salida de PostgreSQL o
MySQL. Solo fija el lenguaje necesario para entender por qué esos motores
exponen planes.

## Ejemplos Progresivos

Los ejemplos del capítulo viven en `examples/` y se pueden ejecutar con
`cargo run --example <nombre>`.

| Ejemplo | Propósito |
|---------|-----------|
| `query_optimizer_basic` | Construir un plan lógico `Project -> Select -> ReadRelation`. |
| `query_optimizer_intermediate` | Comparar rutas físicas `TableScan` e `IndexScan`. |
| `query_optimizer_advanced` | Estimar costo y elegir el plan de menor trabajo. |

## Ejercicios

Los ejercicios están graduados para practicar una idea por vez. Las soluciones
ejecutables viven en `examples/soluciones/`.

### Nivel 1: Table Scan

Objetivo: representar una lectura completa de relación.

Tareas:

- crear `RelationName::new("customers")`;
- construir `PhysicalPlan::table_scan`;
- confirmar que la ruta de acceso es `PhysicalAccessPath::TableScan`;
- explicar por qué no necesita estadísticas de índice.

Solución: `cargo run --example query_optimizer_table_scan`.

### Nivel 2: Index Scan

Objetivo: representar una lectura mediante índice nombrado.

Tareas:

- crear `RelationName::new("customers")`;
- crear `IndexName::new("idx_customers_email")`;
- crear `ColumnName::new("email")`;
- construir `PhysicalPlan::index_scan`;
- confirmar que el plan conserva índice y columna de búsqueda.

Solución: `cargo run --example query_optimizer_index_scan`.

### Nivel 3: Comparación de costo

Objetivo: comparar table scan e index scan con estadísticas educativas.

Tareas:

- declarar `RowCount::new(50_000)`;
- declarar selectividad de 20 puntos base para `idx_customers_email`;
- estimar costo de table scan;
- estimar costo de index scan;
- confirmar que el index scan tiene menor número de unidades de trabajo;
- explicar qué cambiaría si la selectividad fuera baja.

Solución: `cargo run --example query_optimizer_cost_choice`.

## Benchmark Manual

El benchmark del capítulo mide el costo de estimar table scan, index scan y
comparar ambos planes:

```bash
cargo bench --bench query_optimizer_bench
```

La medición no pretende evaluar un optimizador de producción. Sirve para
conectar el modelo conceptual con operaciones repetibles y visibles.

## Lo que aún no hace

Este capítulo todavía no decide:

- cómo parsear SQL;
- cómo transformar automáticamente un plan lógico en varios planes físicos;
- cómo usar histogramas, cardinalidad por columna o correlación entre columnas;
- cómo reproducir la salida exacta de `EXPLAIN` en PostgreSQL, MySQL u otro
  motor real.

## Siguiente paso natural

El siguiente paso natural fuera de este capítulo es volver al capítulo 02, LSM
Tree, y revisar la marca del capítulo 03 en el checklist general.
