# MVCC

> **Estado:** draft.
> **Alcance actual:** representación de versiones de registro, timestamps
> lógicos y metadatos de visibilidad. Snapshot reads, reglas completas de
> visibilidad y comparación con PostgreSQL quedan para los siguientes pasos del
> capítulo.

## Por qué existe

MVCC significa *Multi-Version Concurrency Control*. La idea central es sencilla:
un registro lógico no tiene que ser una sola celda mutable. Puede tener varias
versiones, y cada transacción observa la versión que corresponde a su momento
de lectura.

Sin MVCC, una actualización concurrente suele empujar al motor hacia bloqueos:
si alguien escribe `accounts/42`, otros lectores pueden tener que esperar para
no ver un estado intermedio. Con MVCC, el motor conserva versiones anteriores y
puede permitir que lectores sigan avanzando mientras una escritura produce una
versión nueva.

Este primer paso no implementa todavía snapshots. Antes de decidir qué versión
ve una transacción, el curso necesita una representación explícita de lo que
existe:

- cuál es el registro lógico;
- cuál es la versión concreta;
- cuándo nació la versión;
- si la versión ya fue cerrada por un borrado lógico;
- qué valor guarda esa versión.

## Modelo mental

```text
registro lógico: accounts/42

v1 nace en t10 con saldo=100
v1 se cierra en t12
v2 nace en t12 con saldo=120
```

Una versión no se borra físicamente en este modelo inicial. Queda marcada con
un timestamp de cierre. Esa diferencia importa porque un lector antiguo podría
seguir necesitando la versión anterior, mientras un lector nuevo debería ver la
versión más reciente.

## Modelo Rust actual

El módulo `src/mvcc.rs` expone estos tipos:

| Tipo | Responsabilidad |
|------|-----------------|
| `RecordId` | Identifica el registro lógico, por ejemplo `accounts/42`. |
| `RecordValue` | Guarda el valor educativo asociado a una versión. |
| `VersionId` | Identifica una versión dentro de una cadena. |
| `LogicalTimestamp` | Ordena creación y cierre de versiones. |
| `RecordVersion` | Representa una versión concreta con metadatos de visibilidad. |
| `VersionChain` | Agrupa versiones de un mismo registro lógico en orden de creación. |
| `MvccError` | Nombra violaciones de invariantes del modelo. |

El diseño mantiene el valor como texto para no mezclar MVCC con serialización,
tipos SQL o formatos físicos de página. Es una decisión deliberada: este
capítulo está enseñando visibilidad, no almacenamiento de filas.

## Invariantes

El modelo actual defiende estas reglas:

- `RecordId` no acepta texto vacío después de recortar espacios.
- `RecordValue` no acepta texto vacío después de recortar espacios.
- una `RecordVersion` nace con `created_at` y sin `deleted_at`;
- `delete_at` no puede usar un timestamp anterior al de creación;
- una versión solo puede cerrarse una vez;
- una `VersionChain` solo acepta timestamps de creación monótonos;
- los `VersionId` asignados por `VersionChain::append` son secuenciales desde
  `1`.

Estas invariantes son pequeñas, pero fijan la frontera mental del capítulo. Una
cadena de versiones desordenada vuelve ambiguas las lecturas por snapshot; una
versión que se borra dos veces vuelve confusa la historia; un registro sin
identidad estable no puede indexarse ni compararse.

## Diagrama

```mermaid
flowchart LR
    R["RecordId: accounts/42"]
    V1["VersionId 1<br/>created_at: t10<br/>deleted_at: t12<br/>saldo=100"]
    V2["VersionId 2<br/>created_at: t12<br/>deleted_at: none<br/>saldo=120"]

    R --> V1
    V1 --> V2
```

El diagrama muestra una actualización como cierre de una versión y creación de
otra. En un motor real, el cierre puede representarse con metadatos de
transacción, timestamps, referencias a undo o reglas específicas del motor. En
este curso lo reducimos a timestamps lógicos para que la idea sea visible.

## Ejemplo básico

```rust
use rust_database_internals::mvcc::{
    LogicalTimestamp, RecordId, RecordValue, VersionChain,
};

let record_id = RecordId::new("accounts/42")?;
let mut chain = VersionChain::new(record_id);

let first = chain.append(
    LogicalTimestamp::new(10),
    RecordValue::new("saldo=100")?,
)?;
let second = chain.append(
    LogicalTimestamp::new(12),
    RecordValue::new("saldo=120")?,
)?;

assert_eq!(first.value(), 1);
assert_eq!(second.value(), 2);
assert_eq!(chain.latest().unwrap().value().as_str(), "saldo=120");
# Ok::<(), rust_database_internals::mvcc::MvccError>(())
```

El ejemplo todavía consulta la versión más reciente, no la versión visible para
un snapshot. Esa diferencia será el centro del siguiente paso.

## Lo que aún no hace

Este primer borrador no decide:

- qué snapshot obtiene una transacción al comenzar;
- cómo se calcula si una versión es visible para un timestamp dado;
- qué ocurre con versiones de transacciones abortadas;
- cuándo una versión antigua puede ser recolectada;
- cómo se compara este modelo con `xmin`, `xmax` y snapshots de PostgreSQL.

Esa separación evita un error común: querer explicar MVCC completo antes de
tener una representación mínima verificable. Primero se representa la historia;
después se decide qué lector puede observar cada parte de esa historia.

## Siguiente paso natural

El siguiente paso del capítulo es modelar snapshot reads: una lectura recibe un
timestamp lógico de snapshot y elige la versión correcta de una `VersionChain`.
Después se documentará la relación con PostgreSQL como comparación, sin volver
el curso dependiente de PostgreSQL.
