# Especificación: recovery ante crash antes y después de commit

> **Issue:** #42
> **Milestone:** 08 Recovery
> **Estado:** draft.

## Propósito

Este paso inicia el capítulo de Recovery con la decisión más importante al
reiniciar: distinguir cambios confirmados, que deben considerarse para redo, de
cambios no confirmados, que deben considerarse para undo.

## Alcance actual

- `src/recovery.rs` agrega `RecoveryPlan`.
- `RecoveryPlan::from_wal` inspecciona `WriteAheadLog`.
- Transacciones con `Update` y `Commit` quedan en `redo_transactions`.
- Transacciones con `Update` sin `Commit` ni `Rollback` quedan en
  `undo_transactions`.
- Transacciones abiertas sin cambios no requieren trabajo de recovery.
- Transacciones con `Rollback` no quedan como candidatas a redo ni a undo.
- `tests/recovery_test.rs` cubre caídas antes y después de commit.
- `examples/recovery_crash_commit.rs` muestra el flujo ejecutable.
- `docs/08-recovery.md` documenta el modelo inicial.

## Decisión de diseño

Este issue no aplica páginas ni recorre el WAL para replay. Solo construye el
plan. Separar análisis de replay mantiene clara la frontera conceptual:
primero se decide qué debería pasar; después se implementa cómo se aplica.
