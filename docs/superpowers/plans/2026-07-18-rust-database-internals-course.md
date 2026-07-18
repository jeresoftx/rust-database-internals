# Plan de Curso: rust-database-internals

> **Estado:** checklist inicial de implementación.  
> **Repositorio:** `rust-database-internals`  
> **Fecha:** 2026-07-18  
> **Base:** RFC-0001 §10, §14, §15, §16, §17 y §20.

## Contexto

Este curso es el canon de internals de bases de datos dentro de Jeresoft
Academy. Construye modelos educativos en Rust para explicar cómo funcionan los
motores por dentro.

No depende de PostgreSQL, MySQL, MongoDB ni motores reales. Esos motores se
pueden citar como comparación, pero la práctica con motores reales pertenece a
la secuela propuesta `rust-database-systems`.

## Verificación Recurrente

Antes de cada commit importante, cuando aplique:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
```

## Checklist General

- [x] Crear repositorio GitHub `jeresoftx/rust-database-internals`.
- [x] Configurar About en español.
- [x] Configurar topics: `jeresoft-academy`, `rust`, `database-internals`,
  `databases`, `btree`, `lsm-tree`, `transactions`, `mvcc`, `wal`, `recovery`.
- [x] Clonar en `repos/rust-database-internals`.
- [x] Crear estructura inicial conforme a RFC-0001 §15.
- [ ] Desarrollar capítulo 01: B-Tree.
- [ ] Desarrollar capítulo 02: LSM Tree.
- [ ] Desarrollar capítulo 03: Índices.
- [ ] Desarrollar capítulo 04: Transacciones.
- [ ] Desarrollar capítulo 05: ACID.
- [ ] Desarrollar capítulo 06: MVCC.
- [ ] Desarrollar capítulo 07: Write-Ahead Log.
- [ ] Desarrollar capítulo 08: Recovery.
- [ ] Desarrollar capítulo 09: Replicación.
- [ ] Desarrollar capítulo 10: Query Optimizer.

## Task 1: B-Tree

**Files:**

- Create: `docs/01-btree.md`
- Create: `src/btree.rs`
- Modify: `src/lib.rs`
- Create: `tests/btree_test.rs`
- Create: `benches/btree_bench.rs`
- Create: `diagrams/01-btree.mmd`
- Create: `examples/btree_basic.rs`
- Create: `examples/btree_intermediate.rs`
- Create: `examples/btree_advanced.rs`
- Create: `examples/btree_real_case.rs`
- Create: `examples/soluciones/btree_search.rs`
- Create: `examples/soluciones/btree_insert.rs`
- Create: `examples/soluciones/btree_split.rs`
- Modify: `Cargo.toml`
- Modify: `README.md`
- Modify: `ROADMAP.md`

- [ ] Diseñar API mínima: `BTree`, `NodeId`, `Key`, `RecordPointer`,
  `BTreeError`.
- [ ] Escribir test rojo para búsqueda en árbol vacío.
- [ ] Implementar búsqueda.
- [ ] Escribir test rojo para inserción simple.
- [ ] Implementar inserción sin split.
- [ ] Escribir test rojo para split de nodo.
- [ ] Implementar split educativo.
- [ ] Documentar invariantes: orden, fanout, altura balanceada, claves
  separadoras y punteros.
- [ ] Crear diagrama Mermaid de nodo raíz, hojas y split.
- [ ] Crear ejemplos progresivos y caso real de índice por primary key.
- [ ] Crear ejercicios y soluciones.
- [ ] Crear benchmark de búsqueda e inserción.
- [ ] Actualizar README y ROADMAP a `benchmarked`.
- [ ] Verificar y hacer commit: `feat: add btree chapter`.

## Task 2: LSM Tree

- [ ] Diseñar `MemTable`, `SSTable`, `SegmentId`, `CompactionPlan`,
  `LsmTreeError`.
- [ ] Modelar escrituras en memoria.
- [ ] Modelar flush a segmentos inmutables.
- [ ] Modelar búsqueda entre memtable y segmentos.
- [ ] Modelar compaction educativa.
- [ ] Documentar tradeoffs frente a B-Tree.

## Task 3: Índices

- [ ] Diseñar índices primarios y secundarios.
- [ ] Modelar índice único y no único.
- [ ] Modelar selectividad.
- [ ] Documentar costo de lectura, escritura y mantenimiento.

## Task 4: Transacciones

- [ ] Diseñar `TransactionId`, `TransactionState`, `TransactionManager`.
- [ ] Modelar begin, commit y rollback.
- [ ] Modelar conflictos simples.
- [ ] Documentar atomicidad y aislamiento.

## Task 5: ACID

- [ ] Documentar Atomicity, Consistency, Isolation y Durability.
- [ ] Crear modelos mínimos por propiedad.
- [ ] Diseñar ejercicios de fallas parciales.

## Task 6: MVCC

- [ ] Diseñar versiones de registro.
- [ ] Modelar snapshot reads.
- [ ] Modelar visibilidad por timestamp lógico.
- [ ] Documentar relación con PostgreSQL como comparación.

## Task 7: Write-Ahead Log

- [ ] Diseñar registros de log.
- [ ] Modelar append-only log.
- [ ] Modelar redo y undo educativos.
- [ ] Documentar regla: escribir log antes de modificar página.

## Task 8: Recovery

- [ ] Modelar crash antes y después de commit.
- [ ] Modelar replay del WAL.
- [ ] Documentar checkpoints.

## Task 9: Replicación

- [ ] Modelar primary/replica.
- [ ] Modelar lag.
- [ ] Modelar confirmación síncrona y asíncrona.
- [ ] Documentar tradeoffs de consistencia.

## Task 10: Query Optimizer

- [ ] Diseñar representación de plan lógico y físico.
- [ ] Modelar table scan e index scan.
- [ ] Modelar estimación de costo.
- [ ] Documentar por qué `EXPLAIN` existe en motores reales.
