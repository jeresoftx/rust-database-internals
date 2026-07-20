# Replicación

> **Estado:** draft.
> **Alcance actual:** modelo educativo primary/replica. Solo el primary acepta
> escrituras locales; las réplicas reciben copias ordenadas del WAL del primary.
> Incluye medición educativa de lag por registros pendientes y últimos LSNs.
> Todavía no modela confirmación síncrona/asíncrona ni tradeoffs de consistencia.

## Por qué existe

Replicación existe porque una sola copia de los datos es un punto frágil. Si el
primary cae, si una máquina se pierde o si una región queda inaccesible, el
sistema necesita otra copia suficientemente cercana para seguir razonando.

El precio es que ahora ya no basta preguntar "¿cuál es el estado?". También hay
que preguntar:

- ¿qué nodo aceptó la escritura?
- ¿qué réplicas ya recibieron ese cambio?
- ¿qué tan atrasada está cada réplica?
- ¿cuándo se considera confirmada una escritura?

Este primer paso solo fija el vocabulario: primary, replica y copia ordenada de
registros. El segundo paso agrega lag: una forma explícita de medir qué tan
lejos está una réplica del WAL del primary.

## Modelo mental

```text
primary-1
  WAL: LSN 1 update tx10 saldo=100 -> saldo=120
       LSN 2 commit tx10

replica-1
  WAL: copia LSN 1
       copia LSN 2
```

La réplica no inventa escrituras locales. Sigue al primary copiando su historia
en el mismo orden lógico.

## Modelo Rust actual

El módulo `src/replication.rs` expone estos tipos:

| Tipo | Responsabilidad |
|------|-----------------|
| `ReplicationRole` | Rol del nodo: `Primary` o `Replica`. |
| `ReplicationNode` | Nodo con identificador, rol y WAL local. |
| `ReplicationCluster` | Conjunto educativo con un primary y réplicas. |
| `ReplicationLag` | Atraso observable de una réplica respecto al primary. |
| `ReplicationReport` | Resultado de copiar registros hacia una réplica. |
| `ReplicationError` | Errores del modelo de replicación. |

El primary puede agregar operaciones locales al WAL. Una réplica rechaza
escrituras locales porque, en este modelo, su trabajo es recibir la historia del
primary.

`ReplicationCluster::replica_lag` compara el WAL del primary con el WAL de una
réplica. En este modelo, lag significa registros pendientes de copiar:

```text
primary tiene 2 registros
replica tiene 0 registros
lag = 2 registros pendientes
```

## Invariantes

- un identificador de nodo no puede estar vacío;
- solo un nodo con rol `Primary` acepta escrituras locales;
- un nodo con rol `Replica` rechaza escrituras locales;
- un cluster tiene exactamente un primary explícito;
- la lista de réplicas solo acepta nodos con rol `Replica`;
- los identificadores de nodos dentro del cluster no se repiten;
- la copia hacia una réplica preserva el orden de LSN del primary;
- el lag baja cuando la réplica copia registros del primary;
- una réplica está al día cuando su número de registros coincide con el del
  primary.

## Diagrama

```mermaid
flowchart LR
    P["primary-1<br/>acepta escritura local"]
    W["WAL primary<br/>LSN 1 update<br/>LSN 2 commit"]
    R["replica-1<br/>recibe copia"]
    RW["WAL replica<br/>LSN 1 update<br/>LSN 2 commit"]
    L["lag<br/>0 registros pendientes"]

    P --> W
    W --> R
    R --> RW
    RW --> L
```

## Ejemplo básico

```rust
use rust_database_internals::{
    replication::{ReplicationCluster, ReplicationNode},
    wal::{LogOperation, PageId, PageImage, WalTransactionId},
};

let mut primary = ReplicationNode::primary("primary-1")?;
let tx = WalTransactionId::new(10);

primary.append_local_update(
    tx,
    LogOperation::update(
        PageId::new("heap/accounts/0001")?,
        PageImage::new("saldo=100")?,
        PageImage::new("saldo=120")?,
    )?,
)?;
primary.append_local_commit(tx)?;

let replica = ReplicationNode::replica("replica-1")?;
let mut cluster = ReplicationCluster::new(primary, vec![replica])?;

let report = cluster.replicate_to("replica-1")?;

assert_eq!(report.copied_records(), 2);
assert_eq!(
    cluster.replica("replica-1").expect("réplica").log().last_lsn(),
    cluster.primary().log().last_lsn()
);
# Ok::<(), rust_database_internals::replication::ReplicationError>(())
```

Ejemplo ejecutable: `cargo run --example replication_primary_replica`.

## Lag

Lag nombra la distancia entre lo que el primary ya conoce y lo que una réplica
ha recibido.

```rust
use rust_database_internals::{
    replication::{ReplicationCluster, ReplicationNode},
    wal::{LogOperation, PageId, PageImage, WalTransactionId},
};

let mut primary = ReplicationNode::primary("primary-1")?;
let tx = WalTransactionId::new(10);

primary.append_local_update(
    tx,
    LogOperation::update(
        PageId::new("heap/accounts/0001")?,
        PageImage::new("saldo=100")?,
        PageImage::new("saldo=120")?,
    )?,
)?;
primary.append_local_commit(tx)?;

let replica = ReplicationNode::replica("replica-1")?;
let mut cluster = ReplicationCluster::new(primary, vec![replica])?;

let before = cluster.replica_lag("replica-1")?;
assert_eq!(before.pending_records(), 2);
assert!(!before.is_caught_up());

cluster.replicate_to("replica-1")?;

let after = cluster.replica_lag("replica-1")?;
assert_eq!(after.pending_records(), 0);
assert!(after.is_caught_up());
# Ok::<(), rust_database_internals::replication::ReplicationError>(())
```

Ejemplo ejecutable: `cargo run --example replication_lag`.

## Lo que aún no hace

Este borrador todavía no decide:

- cómo confirmar escrituras de forma síncrona o asíncrona;
- qué ocurre si una réplica pierde registros;
- cómo elegir un nuevo primary;
- cómo razonar sobre consistencia de lecturas desde réplicas.

## Siguiente paso natural

El siguiente paso del capítulo es modelar confirmación síncrona y asíncrona:
cuándo una escritura se considera aceptada si las réplicas van por detrás.
