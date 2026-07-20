# Especificación: lag de replicación

> **Issue:** #46
> **Milestone:** 09 Replicación
> **Estado:** draft.

## Propósito

Este paso modela lag como distancia observable entre el WAL del primary y el WAL
de una réplica.

## Alcance actual

- `ReplicationLag` resume registros del primary, registros de la réplica,
  registros pendientes y últimos LSNs.
- `ReplicationCluster::replica_lag` calcula lag para una réplica conocida.
- Una réplica está al día cuando `pending_records` es `0`.
- `tests/replication_test.rs` cubre lag antes y después de replicar.
- `examples/replication_lag.rs` muestra el flujo ejecutable.
- `docs/09-replicacion.md` documenta el concepto.

## Decisión de diseño

El lag se mide por registros pendientes, no por tiempo de pared. Es una
reducción deliberada: primero importa ver la diferencia entre historias WAL.
La medición temporal, redes, heartbeats y confirmaciones pertenecen a pasos
posteriores del capítulo.
