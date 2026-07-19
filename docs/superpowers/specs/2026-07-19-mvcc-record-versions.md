# Especificación: versiones de registro MVCC

> **Issue:** #34
> **Milestone:** 06 MVCC
> **Estado:** draft.

## Propósito

Este paso abre el capítulo MVCC con la representación mínima de versiones de
registro. Antes de enseñar snapshot reads o visibilidad por timestamp lógico, el
curso necesita una forma explícita de hablar de la historia de un registro:
versiones creadas, versiones cerradas y valores observables.

## Alcance actual

- `src/mvcc.rs` define `RecordId`, `RecordValue`, `VersionId`,
  `LogicalTimestamp`, `RecordVersion`, `VersionChain` y `MvccError`.
- `tests/mvcc_test.rs` cubre creación, normalización, metadatos de visibilidad,
  borrado lógico y orden monótono de versiones.
- `docs/06-mvcc.md` introduce MVCC como representación de historia antes de
  snapshot reads.
- README y ROADMAP muestran MVCC en estado `draft`.
- El checklist del plan marca completado el diseño de versiones de registro.

## Invariantes

- Un `RecordId` vacío se rechaza con `MvccError::BlankRecordId`.
- Un `RecordValue` vacío se rechaza con `MvccError::BlankRecordValue`.
- Una `RecordVersion` nace sin `deleted_at`.
- `delete_at` rechaza timestamps anteriores a `created_at`.
- Una versión cerrada no puede cerrarse por segunda vez.
- `VersionChain::append` asigna `VersionId` secuenciales.
- `VersionChain::append` rechaza timestamps de creación anteriores al último
  timestamp aceptado.

## Decisión de diseño

MVCC se implementa por capas. Este paso solo representa versiones porque
mezclar representación, snapshot reads, reglas de visibilidad y comparación con
PostgreSQL haría más difícil detectar qué invariante se rompió.

El valor de una versión se modela como `RecordValue(String)` en vez de una fila
tipada o una página física. Esa reducción mantiene el foco en el mecanismo:
varias versiones de un mismo registro lógico pueden coexistir y cada versión
tiene metadatos suficientes para razonar sobre su ciclo de vida.

El capítulo queda en estado `draft`, no `implemented`, porque MVCC todavía
necesita snapshot reads, visibilidad por timestamp lógico y una comparación
cuidadosa con PostgreSQL antes de considerarse completo.
