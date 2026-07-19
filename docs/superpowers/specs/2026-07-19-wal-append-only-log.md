# Especificación: append-only log WAL

> **Issue:** #39
> **Milestone:** 07 Write-Ahead Log
> **Estado:** draft.

## Propósito

Este paso agrega una secuencia append-only en memoria para los registros WAL.
Después de diseñar `LogRecord`, el curso necesita una estructura que preserve
orden, asigne LSN monótonos y permita recorrer la historia registrada.

## Alcance actual

- `WriteAheadLog` almacena registros en orden de append.
- `append` asigna el siguiente `LogSequenceNumber` de forma monótona.
- `append_begin`, `append_commit` y `append_rollback` son helpers para
  operaciones comunes.
- `append_record` acepta registros externos solo si su LSN coincide con
  `next_lsn`.
- `tests/wal_test.rs` cubre orden, LSN monótono y rechazo de LSN inesperado.
- `examples/wal_append_only.rs` muestra una historia `begin -> update ->
  commit`.
- `docs/07-write-ahead-log.md` documenta el append-only log.
- El checklist del plan marca completado el modelado de append-only log.

## Invariantes

- El primer LSN asignado por un `WriteAheadLog` nuevo es `1`.
- Cada append avanza `next_lsn` exactamente una posición.
- `records` conserva el orden de inserción.
- `last_lsn` refleja el último registro escrito.
- `append_record` rechaza huecos o adelantos de LSN con
  `WalError::UnexpectedLsn`.

## Decisión de diseño

El log sigue siendo en memoria. No escribe archivos, no sincroniza a disco y no
declara durabilidad. Esa separación es intencional: este paso enseña orden y
append-only; la regla WAL completa y la durabilidad pertenecen a pasos
posteriores.
