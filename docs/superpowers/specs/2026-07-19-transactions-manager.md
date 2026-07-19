# Especificación: TransactionManager Educativo

> **Issue:** #27  
> **Milestone:** 04 Transacciones  
> **Estado:** borrador técnico inicial.

## Propósito

La primera frontera del capítulo de Transacciones define identidad, estado y
registro. No intenta todavía resolver atomicidad ni aislamiento; prepara el
lenguaje para modelar operaciones posteriores.

## API Actual

- `TransactionId`: identificador lógico comparable y ordenable.
- `TransactionState`: `Active`, `Committed` o `RolledBack`.
- `TransactionManager`: registro de transacciones y sus estados.
- `TransactionError`: errores futuros del modelo educativo.

## Invariantes

- `TransactionManager::new` inicia vacío.
- `TransactionManager::next_transaction_id` inicia en `TransactionId(1)`.
- `TransactionManager::register` asigna el siguiente identificador disponible.
- Registrar una transacción incrementa el siguiente identificador.
- `TransactionManager::state` devuelve `Some(state)` para transacciones
  conocidas.
- `TransactionManager::state` devuelve `None` para transacciones desconocidas.
- `TransactionState::as_str` expone nombres estables: `active`, `committed` y
  `rolled_back`.

## Decisión De Diseño

El método `register` recibe un estado inicial explícito porque este issue solo
diseña representación. El siguiente paso puede construir `begin`, `commit` y
`rollback` encima de esta base sin cambiar el contrato central de identidad y
estado.
