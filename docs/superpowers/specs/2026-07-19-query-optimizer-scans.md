# Especificación: table scan e index scan

> **Issue:** #50
> **Milestone:** 10 Query Optimizer
> **Estado:** tested.

## Propósito

Este paso agrega las dos rutas de acceso físicas mínimas que un optimizador
educativo necesita comparar después: leer toda la relación o leer mediante un
índice.

## Alcance actual

- `IndexName` valida nombres de índices disponibles.
- `PhysicalAccessPath::TableScan` representa lectura completa de una relación.
- `PhysicalAccessPath::IndexScan` representa lectura mediante un índice
  nombrado y una columna de búsqueda.
- `PhysicalPlan::table_scan` y `PhysicalPlan::index_scan` crean hojas físicas
  explícitas para cada alternativa.
- `tests/query_optimizer_test.rs` cubre ambas rutas e invariantes de nombres.
- `docs/10-query-optimizer.md` explica la diferencia conceptual sin introducir
  costo ni elección automática.

## Decisión de diseño

El index scan no ejecuta una búsqueda real ni consulta estructuras de índices
del capítulo 03. En este punto solo representa una alternativa física. La
estimación de costo y la decisión entre rutas pertenecen al issue #51.
