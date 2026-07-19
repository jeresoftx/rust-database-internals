# Especificación: registros de Write-Ahead Log

> **Issue:** #38
> **Milestone:** 07 Write-Ahead Log
> **Estado:** draft.

## Propósito

Este paso abre el capítulo Write-Ahead Log con la representación mínima de
registros. Antes de modelar un log append-only, redo, undo o recovery, el curso
necesita una unidad clara de historia: LSN, transacción, página y operación.

## Alcance actual

- `src/wal.rs` define `LogSequenceNumber`, `WalTransactionId`, `PageId`,
  `PageImage`, `LogOperation`, `LogRecord` y `WalError`.
- `tests/wal_test.rs` cubre identificadores, normalización, operaciones
  terminales, updates redoable/undoable y rechazo de actualizaciones sin delta.
- `docs/07-write-ahead-log.md` introduce WAL desde registros individuales.
- README y ROADMAP muestran Write-Ahead Log en estado `draft`.
- El checklist del plan marca completado el diseño de registros de log.

## Invariantes

- Un `PageId` vacío se rechaza con `WalError::BlankPageId`.
- Un `PageImage` vacío se rechaza con `WalError::BlankPageImage`.
- `LogOperation::update` rechaza imágenes `before` y `after` idénticas.
- `LogRecord` conserva LSN, transacción y operación.
- Solo `Update` es redoable y undoable en este primer modelo.
- `Begin`, `Commit` y `Rollback` son operaciones terminales o de ciclo de vida,
  no deltas de página.

## Decisión de diseño

El diseño usa imágenes de página como texto para no mezclar este paso con
serialización, buffer pool, páginas físicas, checksums ni I/O. Esa reducción
mantiene el foco en la pregunta central: qué debe contener un registro WAL para
que después se pueda razonar sobre redo y undo.

El capítulo queda en estado `draft`, no `tested`, porque aún falta modelar el
log append-only, redo/undo educativos y la regla de escribir el log antes de
modificar una página.
