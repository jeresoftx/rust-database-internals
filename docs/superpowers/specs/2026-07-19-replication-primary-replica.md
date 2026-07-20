# Especificación: primary/replica

> **Issue:** #45
> **Milestone:** 09 Replicación
> **Estado:** draft.

## Propósito

Este paso inicia el capítulo de Replicación con el contrato primary/replica:
solo el primary acepta escrituras locales y las réplicas reciben copias
ordenadas del WAL.

## Alcance actual

- `src/replication.rs` agrega `ReplicationRole`, `ReplicationNode`,
  `ReplicationCluster`, `ReplicationReport` y `ReplicationError`.
- `ReplicationNode::primary` crea nodos que aceptan escrituras locales.
- `ReplicationNode::replica` crea nodos que rechazan escrituras locales.
- `ReplicationCluster::replicate_to` copia registros pendientes del primary
  hacia una réplica.
- `tests/replication_test.rs` cubre primary, replica y copia del WAL.
- `examples/replication_primary_replica.rs` muestra el flujo ejecutable.
- `docs/09-replicacion.md` documenta el modelo inicial.

## Decisión de diseño

Este issue no modela lag ni confirmación síncrona/asíncrona. La frontera
deliberada es enseñar primero quién acepta escrituras y cómo se copia una
historia ordenada antes de hablar de atraso, quorum o consistencia.
