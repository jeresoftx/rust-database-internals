# Especificación: replay educativo del WAL

> **Issue:** #43
> **Milestone:** 08 Recovery
> **Estado:** draft.

## Propósito

Este paso convierte el plan de recovery en acciones observables sobre páginas:
redo para transacciones confirmadas y undo para transacciones no confirmadas.

## Alcance actual

- `RecoveryPlan::replay` recibe un `WriteAheadLog` y un `PageStore`.
- Redo recorre el WAL hacia adelante y aplica registros `Update` de
  transacciones confirmadas.
- Undo recorre el WAL hacia atrás y aplica registros `Update` de transacciones
  no confirmadas.
- `RecoveryReport` resume cuántos registros se rehicieron y deshicieron.
- `tests/recovery_test.rs` cubre redo, undo y orden inverso de undo.
- `examples/recovery_replay_wal.rs` muestra una recuperación con una
  transacción confirmada y otra no confirmada.
- `docs/08-recovery.md` documenta el replay educativo.

## Decisión de diseño

El replay se apoya en `PageStore::redo` y `PageStore::undo` para no duplicar la
semántica de páginas. Este paso todavía no implementa checkpoints, disco real,
`fsync`, análisis por LSN ni fases completas de ARIES.
