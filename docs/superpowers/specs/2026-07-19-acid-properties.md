# Especificación: Propiedades ACID

> **Issues:** #31, #32, #33
> **Milestone:** 05 ACID
> **Estado:** benchmarked.

## Propósito

El capítulo ACID documenta Atomicity, Consistency, Isolation y Durability desde
internals, agrega modelos Rust mínimos para cada propiedad y cierra con
ejercicios de fallas parciales. Fija el vocabulario para que los capítulos de
WAL, Recovery y MVCC puedan profundizar sin mezclar promesas con mecanismos.

## Alcance Actual

- `docs/05-acid.md` explica las cuatro propiedades.
- Atomicity se relaciona con ciclo de vida, undo y redo.
- Consistency se relaciona con invariantes, constraints e índices.
- Isolation se relaciona con locks, conflictos y MVCC futuro.
- Durability se relaciona con WAL, fsync, checkpoints y recovery.
- `src/acid.rs` expone `AcidProperty`, `AtomicUnit`, `UniqueConstraint`,
  `ReadCommittedCell` y `CommitLog`.
- `tests/acid_test.rs` cubre las invariantes mínimas por propiedad.
- Ejemplos progresivos: `acid_basic`, `acid_intermediate` y `acid_advanced`.
- Soluciones ejecutables: `acid_atomicity_failure`,
  `acid_consistency_failure`, `acid_isolation_failure` y
  `acid_durability_failure`.
- Benchmark manual: `acid_bench`.
- Diagrama Mermaid fuente: `diagrams/05-acid.mmd`.
- README y ROADMAP muestran ACID como capítulo en estado `benchmarked`.
- El checklist del plan marca completada la documentación de propiedades.
- El checklist del plan marca completados los modelos mínimos por propiedad.
- El checklist del plan marca completados ejercicios, soluciones, diagrama y
  benchmark.

## Decisión De Diseño

ACID se documentó antes de modelarse porque sus propiedades son promesas del
motor, no estructuras de datos aisladas. Los modelos mínimos se agregan sin
cambiar esa narrativa: cada modelo enseña una forma concreta de defender o
romper una promesa.

Los modelos mínimos evitan implementar WAL, MVCC o recovery real. `AtomicUnit`
enseña cierre todo-o-nada, `UniqueConstraint` enseña defensa de invariantes,
`ReadCommittedCell` enseña visibilidad de valor confirmado y `CommitLog` enseña
la diferencia entre registrar y sincronizar un commit.

Los ejercicios de fallas parciales usan esos mismos modelos en escenarios
pequeños: rollback antes de commit, rechazo de duplicados, escritura pendiente
no visible y commit no durable antes de `sync`. El capítulo se marca
`benchmarked`, no `reviewed`, porque todavía requiere revisión humana antes de
considerarse publicable.
