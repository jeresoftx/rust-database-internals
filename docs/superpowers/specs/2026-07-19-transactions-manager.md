# Especificación: TransactionManager Educativo

> **Issues:** #27, #28, #29
> **Milestone:** 04 Transacciones
> **Estado:** borrador técnico de ciclo de vida y conflictos simples.

## Propósito

La primera frontera del capítulo de Transacciones define identidad, estado,
registro, transiciones mínimas y conflictos simples. No intenta todavía
resolver atomicidad ni aislamiento; prepara el lenguaje para modelar
operaciones posteriores.

## API Actual

- `TransactionId`: identificador lógico comparable y ordenable.
- `ResourceId`: recurso lógico comparable y ordenable.
- `TransactionState`: `Active`, `Committed` o `RolledBack`.
- `TransactionManager`: registro de transacciones, sus estados y sus locks.
- `TransactionManager::begin`: abre una transacción activa.
- `TransactionManager::commit`: cierra una transacción activa como confirmada.
- `TransactionManager::rollback`: cierra una transacción activa como revertida.
- `TransactionManager::lock_exclusive`: reserva un recurso para una transacción
  activa.
- `TransactionManager::lock_owner`: consulta qué transacción conserva un
  recurso.
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
- `ResourceId::new` rechaza nombres vacíos.
- `TransactionManager::lock_exclusive` exige que la transacción exista y esté
  activa.
- Un recurso exclusivo solo puede tener un dueño a la vez.
- La misma transacción puede volver a tomar el mismo recurso sin duplicar el
  lock.
- `TransactionManager::commit` libera los locks de la transacción confirmada.
- `TransactionManager::rollback` libera los locks de la transacción revertida.
- `TransactionManager::state` devuelve `Some(state)` para transacciones
  conocidas.
- `TransactionManager::state` devuelve `None` para transacciones desconocidas.
- Cerrar una transacción desconocida devuelve
  `TransactionError::UnknownTransaction`.
- Cerrar una transacción terminal devuelve
  `TransactionError::InvalidStateTransition`.
- Intentar tomar un lock desde una transacción cerrada devuelve
  `TransactionError::InactiveTransaction`.
- Intentar tomar un recurso ocupado por otra transacción devuelve
  `TransactionError::ResourceConflict`.
- `TransactionState::as_str` expone nombres estables: `active`, `committed` y
  `rolled_back`.

## Decisión De Diseño

El método `register` conserva un estado inicial explícito porque sirve como
primitiva educativa de representación. `begin`, `commit` y `rollback` se
construyen encima de esa base: `begin` fija el estado inicial común y las
operaciones de cierre centralizan la validación de estados terminales.

Los conflictos simples usan locks exclusivos por `ResourceId`. Esta decisión
mantiene el foco del capítulo en la representación del conflicto: quién tiene
un recurso, quién lo pide y cuándo se libera. No modela colas, espera,
deadlocks, lock escalation ni niveles de aislamiento; esas decisiones pertenecen
a pasos posteriores del curso.
