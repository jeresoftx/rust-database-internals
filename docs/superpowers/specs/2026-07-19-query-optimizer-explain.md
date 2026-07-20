# Especificación: EXPLAIN, ejemplos, ejercicios y benchmark

> **Issue:** #52
> **Milestone:** 10 Query Optimizer
> **Estado:** benchmarked.

## Propósito

Este paso cierra el capítulo de Query Optimizer con la relación conceptual
entre planes físicos, costo estimado y la razón de existir de `EXPLAIN` en
motores reales.

## Alcance actual

- `docs/10-query-optimizer.md` explica `EXPLAIN` como ventana hacia la
  hipótesis del motor.
- El capítulo distingue plan, ruta de acceso, filas estimadas y unidades de
  trabajo.
- Se agregan ejemplos progresivos:
  - `query_optimizer_basic`;
  - `query_optimizer_intermediate`;
  - `query_optimizer_advanced`.
- Se agregan soluciones ejecutables:
  - `query_optimizer_table_scan`;
  - `query_optimizer_index_scan`;
  - `query_optimizer_cost_choice`.
- Se agrega `benches/query_optimizer_bench.rs`.
- README, ROADMAP y checklist elevan Query Optimizer a `benchmarked`.

## Decisión de diseño

El capítulo no implementa SQL, parser, reescritura de consultas ni una salida
compatible con PostgreSQL o MySQL. `EXPLAIN` se documenta como concepto:
mostrar qué plan cree conveniente el motor y con qué estadísticas lo justifica.
El benchmark mide el modelo educativo, no desempeño de un motor real.
