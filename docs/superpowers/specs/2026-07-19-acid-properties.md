# Especificación: Propiedades ACID

> **Issues:** #31, #32
> **Milestone:** 05 ACID
> **Estado:** borrador técnico con modelos mínimos.

## Propósito

El capítulo ACID documenta Atomicity, Consistency, Isolation y Durability desde
internals y agrega modelos Rust mínimos para cada propiedad. Todavía no
implementa simulaciones de fallas parciales; fija el vocabulario para que esos
ejercicios no mezclen promesas con mecanismos.

## Alcance Actual

- `docs/05-acid.md` explica las cuatro propiedades.
- Atomicity se relaciona con ciclo de vida, undo y redo.
- Consistency se relaciona con invariantes, constraints e índices.
- Isolation se relaciona con locks, conflictos y MVCC futuro.
- Durability se relaciona con WAL, fsync, checkpoints y recovery.
- `src/acid.rs` expone `AcidProperty`, `AtomicUnit`, `UniqueConstraint`,
  `ReadCommittedCell` y `CommitLog`.
- `tests/acid_test.rs` cubre las invariantes mínimas por propiedad.
- README y ROADMAP muestran ACID como capítulo en estado `draft`.
- El checklist del plan marca completada la documentación de propiedades.
- El checklist del plan marca completados los modelos mínimos por propiedad.

## Decisión De Diseño

ACID se documentó antes de modelarse porque sus propiedades son promesas del
motor, no estructuras de datos aisladas. Los modelos mínimos se agregan sin
cambiar esa narrativa: cada modelo enseña una forma concreta de defender o
romper una promesa.

Los modelos mínimos evitan implementar WAL, MVCC o recovery real. `AtomicUnit`
enseña cierre todo-o-nada, `UniqueConstraint` enseña defensa de invariantes,
`ReadCommittedCell` enseña visibilidad de valor confirmado y `CommitLog` enseña
la diferencia entre registrar y sincronizar un commit.
