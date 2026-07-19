# Especificación: TransactionManager Educativo

> **Issues:** #27, #28
> **Milestone:** 04 Transacciones
> **Estado:** borrador técnico de ciclo de vida.

## Propósito

La primera frontera del capítulo de Transacciones define identidad, estado,
registro y transiciones mínimas. No intenta todavía resolver atomicidad ni
aislamiento; prepara el lenguaje para modelar operaciones posteriores.

## API Actual

- `TransactionId`: identificador lógico comparable y ordenable.
- `TransactionState`: `Active`, `Committed` o `RolledBack`.
- `TransactionManager`: registro de transacciones y sus estados.
- `TransactionManager::begin`: abre una transacción activa.
- `TransactionManager::commit`: cierra una transacción activa como confirmada.
- `TransactionManager::rollback`: cierra una transacción activa como revertida.
- `TransactionError`: errores del modelo educativo.

## Invariantes

- `TransactionManager::new` inicia vacío.
- `TransactionManager::next_transaction_id` inicia en `TransactionId(1)`.
- `TransactionManager::register` asigna el siguiente identificador disponible.
- Registrar una transacción incrementa el siguiente identificador.
- `TransactionManager::begin` registra una transacción en estado `Active`.
- `TransactionManager::commit` solo transiciona de `Active` a `Committed`.
- `TransactionManager::rollback` solo transiciona de `Active` a `RolledBack`.
- `Committed` y `RolledBack` son estados terminales.
- `TransactionManager::state` devuelve `Some(state)` para transacciones
  conocidas.
- `TransactionManager::state` devuelve `None` para transacciones desconocidas.
- Cerrar una transacción desconocida devuelve
  `TransactionError::UnknownTransaction`.
- Cerrar una transacción terminal devuelve
  `TransactionError::InvalidStateTransition`.
- `TransactionState::as_str` expone nombres estables: `active`, `committed` y
  `rolled_back`.

## Decisión De Diseño

El método `register` conserva un estado inicial explícito porque sirve como
primitiva educativa de representación. `begin`, `commit` y `rollback` se
construyen encima de esa base: `begin` fija el estado inicial común y las
operaciones de cierre centralizan la validación de estados terminales.
