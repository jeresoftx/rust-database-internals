# Especificación: estimación de costo

> **Issue:** #51
> **Milestone:** 10 Query Optimizer
> **Estado:** tested.

## Propósito

Este paso agrega un modelo mínimo de costo para comparar planes físicos del
query optimizer educativo.

## Alcance actual

- `RowCount` representa conteo de filas de una relación.
- `Selectivity` representa selectividad de índice en puntos base.
- `RelationStatistics` e `IndexStatistics` describen estadísticas mínimas.
- `CostCatalog` concentra estadísticas de relaciones e índices.
- `PlanCost` expone filas leídas, filas producidas y unidades de trabajo.
- `PhysicalPlan::estimate_cost` estima costo para table scan e index scan.
- `tests/query_optimizer_test.rs` cubre table scan, index scan, selectividad y
  comparación de costos.
- `docs/10-query-optimizer.md` explica la fórmula sin introducir todavía
  `EXPLAIN`.

## Decisión de diseño

El costo es una unidad abstracta, no una medición de tiempo real. Table scan
cuesta una unidad por fila de la relación. Index scan cuesta una búsqueda fija
de índice más las filas estimadas por selectividad. Esta decisión mantiene el
modelo pequeño y permite explicar después por qué un motor real necesita
`EXPLAIN`.
