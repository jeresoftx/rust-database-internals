# Especificación: Propiedades ACID

> **Issue:** #31
> **Milestone:** 05 ACID
> **Estado:** borrador documental.

## Propósito

El primer paso del capítulo ACID documenta Atomicity, Consistency, Isolation y
Durability desde internals. No implementa todavía modelos Rust ni simulaciones
de fallas parciales; fija el vocabulario para que esos modelos no mezclen
promesas con mecanismos.

## Alcance Actual

- `docs/05-acid.md` explica las cuatro propiedades.
- Atomicity se relaciona con ciclo de vida, undo y redo.
- Consistency se relaciona con invariantes, constraints e índices.
- Isolation se relaciona con locks, conflictos y MVCC futuro.
- Durability se relaciona con WAL, fsync, checkpoints y recovery.
- README y ROADMAP muestran ACID como capítulo en estado `draft`.
- El checklist del plan marca completada la documentación de propiedades.

## Decisión De Diseño

ACID se documenta antes de modelarse porque sus propiedades son promesas del
motor, no estructuras de datos aisladas. El siguiente paso puede crear modelos
mínimos por propiedad sin cambiar la narrativa: cada modelo debe enseñar una
forma concreta de defender o romper una promesa.
