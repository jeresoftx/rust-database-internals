# Write-Ahead Log

> **Estado:** draft.
> **Alcance actual:** representación de registros WAL, LSN, transacción lógica,
> página lógica, imagen antes/después, tipos de operación educativos y log
> append-only en memoria. Redo, undo y la regla completa de escribir log antes
> de modificar página quedan para los siguientes pasos del capítulo.

## Por qué existe

Write-Ahead Log existe porque una base de datos no puede confiar solo en el
estado final de sus páginas. Si el proceso cae a mitad de una escritura, el
motor necesita una historia ordenada para responder dos preguntas:

- qué cambios confirmados deben rehacerse;
- qué cambios incompletos deben deshacerse.

La idea central es escribir primero una descripción durable del cambio y
después modificar la página de datos. Por eso se llama *write-ahead*: el log va
delante de la página.

El primer paso fija el vocabulario mínimo para que los siguientes pasos no
mezclen conceptos. El segundo paso agrega un log append-only en memoria:
registros que entran al final, reciben LSN monótono y preservan orden.

## Modelo mental

```text
LSN 1: begin tx10
LSN 2: update tx10 page heap/accounts/0001 before saldo=100 after saldo=120
LSN 3: commit tx10
```

El WAL no guarda "un comentario". Guarda una secuencia ordenada de registros
con suficiente información para reconstruir decisiones después de una falla.

## Modelo Rust actual

El módulo `src/wal.rs` expone estos tipos:

| Tipo | Responsabilidad |
|------|-----------------|
| `LogSequenceNumber` | Posición lógica de un registro WAL. |
| `WalTransactionId` | Transacción lógica asociada a un registro. |
| `PageId` | Página lógica afectada por una actualización. |
| `PageImage` | Imagen educativa antes o después del cambio. |
| `LogOperation` | Operación registrada: `Begin`, `Update`, `Commit`, `Rollback`. |
| `LogRecord` | Registro WAL con LSN, transacción y operación. |
| `WriteAheadLog` | Secuencia append-only de registros en orden de LSN. |
| `WalError` | Errores de representación del modelo WAL. |

El modelo usa texto para representar imágenes de página. Es deliberado: el
capítulo no intenta enseñar todavía layout físico, checksums, buffers ni I/O.
Primero se necesita una unidad clara de historia.

## Invariantes

El modelo actual defiende estas reglas:

- `PageId` no acepta texto vacío después de recortar espacios;
- `PageImage` no acepta texto vacío después de recortar espacios;
- una operación `Update` requiere una imagen `before` y una imagen `after`
  distintas;
- un registro WAL siempre tiene LSN, transacción y operación;
- solo `Update` es redoable y undoable en este modelo inicial;
- `Begin`, `Commit` y `Rollback` nombran transiciones, pero no contienen delta
  de página.
- `WriteAheadLog` asigna LSN desde `1` de forma monótona;
- `WriteAheadLog::append_record` rechaza registros cuyo LSN no coincide con el
  siguiente esperado;
- los registros se recorren en el mismo orden en el que se agregaron.

## Diagrama

```mermaid
flowchart LR
    B["LSN 1<br/>Begin tx10"]
    U["LSN 2<br/>Update tx10<br/>page heap/accounts/0001<br/>before saldo=100<br/>after saldo=120"]
    C["LSN 3<br/>Commit tx10"]

    B --> U
    U --> C
```

El diagrama muestra una historia, no un estado final. Esa distinción prepara el
terreno para redo y undo: `after` permite rehacer, `before` permite deshacer.

## Ejemplo básico

```rust
use rust_database_internals::wal::{
    LogOperation, LogRecord, LogSequenceNumber, PageId, PageImage,
    WalTransactionId,
};

let page_id = PageId::new("heap/accounts/0001")?;
let before = PageImage::new("saldo=100")?;
let after = PageImage::new("saldo=120")?;
let update = LogOperation::update(page_id, before, after)?;

let record = LogRecord::new(
    LogSequenceNumber::new(2),
    WalTransactionId::new(10),
    update,
);

assert!(record.is_redoable());
assert!(record.is_undoable());
# Ok::<(), rust_database_internals::wal::WalError>(())
```

## Append-only log

Un WAL append-only no reescribe su historia. Agrega registros al final y
mantiene el orden. En este modelo, `WriteAheadLog` asigna el siguiente LSN al
hacer `append`:

```rust
use rust_database_internals::wal::{
    LogOperation, PageId, PageImage, WalTransactionId, WriteAheadLog,
};

let mut log = WriteAheadLog::new();
let tx = WalTransactionId::new(10);

let begin = log.append_begin(tx);
let update = LogOperation::update(
    PageId::new("heap/accounts/0001")?,
    PageImage::new("saldo=100")?,
    PageImage::new("saldo=120")?,
)?;
let update_lsn = log.append(tx, update);
let commit = log.append_commit(tx);

assert_eq!(begin.value(), 1);
assert_eq!(update_lsn.value(), 2);
assert_eq!(commit.value(), 3);
# Ok::<(), rust_database_internals::wal::WalError>(())
```

Ejemplo ejecutable: `cargo run --example wal_append_only`.

## Lo que aún no hace

Este borrador todavía no decide:

- cuándo un registro se considera durable;
- cómo se aplica redo;
- cómo se aplica undo;
- cómo se relaciona la regla WAL con páginas sucias en buffer pool;
- cómo se recupera el sistema después de un crash.

## Siguiente paso natural

El siguiente paso del capítulo es modelar redo y undo educativos usando las
imágenes `after` y `before` que ya viven en cada registro `Update`.
