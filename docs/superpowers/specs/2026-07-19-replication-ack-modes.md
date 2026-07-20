# Especificación: confirmación síncrona y asíncrona

> **Issue:** #47
> **Milestone:** 09 Replicación
> **Estado:** draft.

## Propósito

Este paso modela cuándo una escritura replicada se considera confirmada según
dos políticas educativas: asíncrona y síncrona.

## Alcance actual

- `ReplicationAckMode` define `Async` y `Sync`.
- `ReplicationDecision` distingue `Confirmed` de `WaitingForReplicas`.
- `ReplicationCluster::confirm_write` confirma en modo async aunque haya lag.
- `ReplicationCluster::confirm_write` espera en modo sync si alguna réplica
  tiene registros pendientes.
- `tests/replication_test.rs` cubre ambos modos.
- `examples/replication_ack_modes.rs` muestra el flujo ejecutable.
- `docs/09-replicacion.md` documenta confirmación.

## Decisión de diseño

El modo síncrono exige que todas las réplicas conocidas estén al día. Es una
regla conservadora y fácil de explicar. Quorum, mayorías, réplicas opcionales,
timeouts y fallas de red pertenecen a pasos posteriores o a sistemas
distribuidos.
