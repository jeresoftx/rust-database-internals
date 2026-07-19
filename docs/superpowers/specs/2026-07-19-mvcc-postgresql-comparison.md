# Especificación: comparación MVCC con PostgreSQL

> **Issue:** #37
> **Milestone:** 06 MVCC
> **Estado:** tested.

## Propósito

Este paso cierra el bloque MVCC con una comparación educativa contra
PostgreSQL. La comparación ayuda a conectar el modelo Rust del curso con un
motor real sin convertir PostgreSQL en dependencia del repositorio.

## Alcance actual

- `docs/06-mvcc.md` documenta cómo se relacionan `RecordVersion`, `Snapshot`,
  `VisibilityDecision`, `created_at` y `deleted_at` con ideas de PostgreSQL.
- La comparación cubre `xmin`, `xmax`, `ctid`, snapshots por nivel de
  aislamiento, versiones muertas y vacuum.
- README y ROADMAP elevan MVCC a estado `tested`.
- El checklist del plan marca completado el capítulo 06: MVCC.
- No se agrega código nuevo porque el issue es documental.

## Fuentes oficiales

Se consultó la documentación oficial de PostgreSQL 18:

- `13.1 Introduction`: MVCC y snapshots.
- `5.6 System Columns`: `xmin`, `xmax` y `ctid`.
- `13.2 Transaction Isolation`: `Read Committed`, `Repeatable Read` y
  snapshots.
- `24.1 Routine Vacuuming`: versiones muertas y `VACUUM`.

## Decisión de diseño

La comparación es intencionalmente asimétrica. El curso usa timestamps lógicos
simples para enseñar visibilidad; PostgreSQL usa XIDs, estado transaccional,
snapshots reales, páginas heap, columnas de sistema, locks, vacuum y reglas de
aislamiento más ricas.

El capítulo se marca como `tested`, no `reviewed` ni `published`, porque ya
tiene modelos, pruebas, ejemplos y documentación del alcance planeado, pero
todavía requiere revisión humana antes de considerarse publicable.
