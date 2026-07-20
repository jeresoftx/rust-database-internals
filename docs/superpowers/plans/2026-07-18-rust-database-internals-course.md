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

## Flujo De Trabajo Por Paso

Antes de tocar código de curso, este plan debe desarrollarse como milestones e
issues en GitHub. A partir de ese tablero, cada paso se trabaja con la regla:

```text
milestone + issue asignado → 1 commit → PR asignado → revisión humana → merge a main
```

Aplicación práctica:

- Cada task o subtask accionable debe tener un issue propio.
- Cada issue debe estar asignado a `jeresoftx`.
- Cada issue debe pertenecer al milestone del capítulo o fase correspondiente.
- Cada issue debe resolverse en una rama propia.
- Cada PR debe apuntar a `main` y resumir el cambio, la verificación y el
  siguiente paso natural.
- Cada PR debe estar asignado a `jeresoftx`.
- Cada PR debe estar asociado al mismo milestone que el issue que resuelve.
- Cada issue y PR deben llevar labels coherentes de tipo, capítulo o fase, y
  estado. Labels mínimos: `tipo: documentación`, `tipo: funcionalidad`,
  `tipo: prueba`, el label de capítulo correspondiente, `flujo: issue-pr` y
  `estado: revisión`.
- El PR queda abierto hasta que Joel apruebe explícitamente el merge.
- Si un task es demasiado grande para un solo commit, se divide antes de
  comenzar y se crean issues separados.
- No se continúa con el siguiente issue hasta cerrar o pausar explícitamente el
  PR actual.

## Milestones Del Plan

| Milestone | Alcance | Issues |
|-----------|---------|--------|
| 00 Gobernanza y planificación | Reglas de trabajo, checklist operativo y preparación | #1 |
| 01 B-Tree | Búsqueda, inserción, split, invariantes, docs, ejemplos y benchmark | #3-#16 |
| 02 LSM Tree | MemTable, SSTable, búsqueda, flush y compaction | #17-#22 |
| 03 Índices | Primarios, secundarios, únicos, no únicos y selectividad | #23-#26 |
| 04 Transacciones | Manager, begin, commit, rollback y conflictos | #27-#30 |
| 05 ACID | Propiedades, modelos mínimos y fallas parciales | #31-#33 |
| 06 MVCC | Versiones, snapshots, timestamps y comparación con PostgreSQL | #34-#37 |
| 07 Write-Ahead Log | Registros, append-only log, redo, undo y regla WAL | #38-#41 |
| 08 Recovery | Crash, replay del WAL y checkpoints | #42-#44 |
| 09 Replicación | Primary/replica, lag, confirmación y consistencia | #45-#48 |
| 10 Query Optimizer | Plan lógico/físico, scans, costo y `EXPLAIN` | #49-#52 |

## Checklist General

- [x] Crear repositorio GitHub `jeresoftx/rust-database-internals`.
- [x] Configurar About en español.
- [x] Configurar topics: `jeresoft-academy`, `rust`, `database-internals`,
  `databases`, `btree`, `lsm-tree`, `transactions`, `mvcc`, `wal`, `recovery`.
- [x] Clonar en `repos/rust-database-internals`.
- [x] Crear estructura inicial conforme a RFC-0001 §15.
- [x] Documentar flujo issue-commit-PR por paso del plan.
- [x] Crear milestones e issues del plan completo, asignados a `jeresoftx`.
- [x] Documentar milestones, labels y asignación obligatoria para PRs.
- [x] Desarrollar capítulo 01: B-Tree.
- [ ] Desarrollar capítulo 02: LSM Tree.
- [ ] Desarrollar capítulo 03: Índices.
- [x] Desarrollar capítulo 04: Transacciones.
- [x] Desarrollar capítulo 05: ACID.
- [x] Desarrollar capítulo 06: MVCC.
- [x] Desarrollar capítulo 07: Write-Ahead Log.
- [x] Desarrollar capítulo 08: Recovery.
- [x] Desarrollar capítulo 09: Replicación.
- [x] Desarrollar capítulo 10: Query Optimizer.

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

- [x] Diseñar API mínima: `BTree`, `NodeId`, `Key`, `RecordPointer`,
  `BTreeError`.
- [x] Escribir test rojo para búsqueda en árbol vacío.
- [x] Implementar búsqueda.
- [x] Escribir test rojo para inserción simple.
- [x] Implementar inserción sin split.
- [x] Escribir test rojo para split de nodo.
- [x] Implementar split educativo.
- [x] Documentar invariantes: orden, fanout, altura balanceada, claves
  separadoras y punteros.
- [x] Crear diagrama Mermaid de nodo raíz, hojas y split.
- [x] Crear ejemplos progresivos y caso real de índice por primary key.
- [x] Crear ejercicios y soluciones.
- [x] Crear benchmark de búsqueda e inserción.
- [x] Actualizar README y ROADMAP a `benchmarked`.
- [x] Verificar y hacer commit: `docs: close btree verification`.

### Registro De Verificación

El issue #4 queda trazado por la especificación
[`docs/superpowers/specs/2026-07-20-btree-empty-search.md`](../specs/2026-07-20-btree-empty-search.md).
El test `search_in_empty_tree_returns_none` valida el contrato original: buscar
en un B-Tree vacío devuelve `Ok(None)`.

El cierre técnico de B-Tree se prepara en el issue #16. Antes de abrir el PR
final del capítulo se ejecuta la batería completa:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
cargo bench --bench btree_bench
git diff --check
```

Resultado esperado para revisión humana: el capítulo queda en estado
`benchmarked`, con API, pruebas, documentación, ejemplos, ejercicios,
soluciones y benchmark educativo completos. No se marca como `reviewed` ni
`published` hasta que Joel revise y apruebe el bloque.

## Task 2: LSM Tree

- [x] Diseñar `MemTable`, `SSTable`, `SegmentId`, `CompactionPlan`,
  `LsmTreeError`.
- [x] Modelar escrituras en memoria.
- [x] Modelar flush a segmentos inmutables.
- [x] Modelar búsqueda entre memtable y segmentos.
- [x] Modelar compaction educativa.
- [x] Documentar tradeoffs frente a B-Tree.

## Task 3: Índices

- [x] Diseñar índices primarios y secundarios.
- [x] Modelar índice único y no único.
- [x] Modelar selectividad.
- [x] Documentar costo de lectura, escritura y mantenimiento.

## Task 4: Transacciones

- [x] Diseñar `TransactionId`, `TransactionState`, `TransactionManager`.
- [x] Modelar begin, commit y rollback.
- [x] Modelar conflictos simples.
- [x] Documentar atomicidad y aislamiento.
- [x] Crear ejemplos progresivos de transacciones.
- [x] Crear ejercicios y soluciones.
- [x] Crear diagrama Mermaid de ciclo de vida y conflicto.
- [x] Crear benchmark de ciclo de vida, locks y conflictos.
- [x] Actualizar README y ROADMAP a `benchmarked`.

## Task 5: ACID

- [x] Documentar Atomicity, Consistency, Isolation y Durability.
- [x] Crear modelos mínimos por propiedad.
- [x] Diseñar ejercicios de fallas parciales.
- [x] Crear soluciones ejecutables de fallas parciales.
- [x] Crear diagrama Mermaid de flujo ACID.
- [x] Crear benchmark de rollback, constraints, aislamiento y durabilidad.
- [x] Actualizar README y ROADMAP a `benchmarked`.

## Task 6: MVCC

- [x] Diseñar versiones de registro.
- [x] Modelar snapshot reads.
- [x] Modelar visibilidad por timestamp lógico.
- [x] Documentar relación con PostgreSQL como comparación.

## Task 7: Write-Ahead Log

- [x] Diseñar registros de log.
- [x] Modelar append-only log.
- [x] Modelar redo y undo educativos.
- [x] Documentar regla: escribir log antes de modificar página.

## Task 8: Recovery

- [x] Modelar crash antes y después de commit.
- [x] Modelar replay del WAL.
- [x] Documentar checkpoints.

## Task 9: Replicación

- [x] Modelar primary/replica.
- [x] Modelar lag.
- [x] Modelar confirmación síncrona y asíncrona.
- [x] Documentar tradeoffs de consistencia.

## Task 10: Query Optimizer

- [x] Diseñar representación de plan lógico y físico.
- [x] Modelar table scan e index scan.
- [x] Modelar estimación de costo.
- [x] Documentar por qué `EXPLAIN` existe en motores reales.
