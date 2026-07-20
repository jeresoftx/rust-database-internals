# Especificación: redo y undo educativos WAL

> **Issue:** #40
> **Milestone:** 07 Write-Ahead Log
> **Estado:** draft.

## Propósito

Este paso modela redo y undo educativos usando los registros `Update` del WAL.
La meta no es implementar recovery completo, sino hacer visible que la imagen
`after` permite rehacer y la imagen `before` permite deshacer.

## Alcance actual

- `PageStore` almacena imágenes de página en memoria.
- `PageStore::redo` aplica la imagen `after` de un registro `Update`.
- `PageStore::undo` aplica la imagen `before` de un registro `Update`.
- Redo y undo rechazan registros no aplicables con errores explícitos.
- `tests/wal_test.rs` cubre escritura/lectura de páginas, redo, undo y rechazo
  de registros no redoable/undoable.
- `examples/wal_redo_undo.rs` muestra el flujo `before -> after -> before`.
- `docs/07-write-ahead-log.md` documenta redo y undo educativos.
- El checklist del plan marca completado el modelado de redo y undo.

## Invariantes

- Solo registros `Update` pueden aplicar redo.
- Solo registros `Update` pueden aplicar undo.
- Redo escribe exactamente la imagen `after`.
- Undo escribe exactamente la imagen `before`.
- `Begin`, `Commit` y `Rollback` no tienen delta de página y por eso se
  rechazan para redo/undo.

## Decisión de diseño

`PageStore` es un almacén en memoria, no un buffer pool real. No sabe de
durabilidad, páginas sucias, checkpoints ni transacciones confirmadas. Esa
reducción mantiene el foco en la anatomía del registro WAL: el par
`before`/`after` es suficiente para enseñar el corazón de redo y undo antes de
hablar de recovery.
