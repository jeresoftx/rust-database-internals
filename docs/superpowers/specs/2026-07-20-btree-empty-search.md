# Especificación: búsqueda en B-Tree vacío

> **Issue:** #4
> **Milestone:** 01 B-Tree
> **Estado:** benchmarked.

## Propósito

Este paso registra la trazabilidad del primer test rojo de B-Tree: buscar una
clave en un árbol vacío debe devolver `Ok(None)`.

## Alcance actual

- `tests/btree_test.rs` contiene `search_in_empty_tree_returns_none`.
- La prueba confirma que un B-Tree recién creado:
  - está vacío;
  - tiene longitud `0`;
  - tiene altura `0`;
  - responde `Ok(None)` al buscar una clave ausente.
- `docs/superpowers/plans/2026-07-18-rust-database-internals-course.md`
  marca como completa la tarea "Escribir test rojo para búsqueda en árbol
  vacío".

## Decisión de diseño

No se cambia código en este issue porque el comportamiento ya existe y está
probado. Este PR cierra la brecha de trazabilidad entre el issue de GitHub y el
estado real del repositorio.
