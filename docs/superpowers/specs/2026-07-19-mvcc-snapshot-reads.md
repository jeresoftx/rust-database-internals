# Especificación: snapshot reads MVCC

> **Issue:** #35
> **Milestone:** 06 MVCC
> **Estado:** draft.

## Propósito

Este paso agrega lecturas por snapshot al modelo MVCC. La representación de
versiones ya existía; ahora una lectura puede declarar un timestamp lógico y
recibir la versión visible para ese momento.

## Alcance actual

- `Snapshot` representa el timestamp lógico de lectura.
- `RecordVersion::is_visible_in` decide si una versión es visible para un
  snapshot.
- `VersionChain::read` devuelve la versión visible más reciente.
- `VersionChain::delete_latest_at` permite cerrar la versión más reciente para
  modelar actualizaciones o borrados lógicos.
- `tests/mvcc_test.rs` cubre snapshots antes de la primera versión, snapshots
  antiguos, snapshots nuevos y borrados sin reemplazo.
- `examples/mvcc_basic.rs`, `examples/mvcc_intermediate.rs` y
  `examples/mvcc_advanced.rs` documentan el flujo de lectura progresivo.
- `docs/06-mvcc.md` explica la regla de snapshot read.
- El checklist del plan marca completado el modelado de snapshot reads.

## Regla de visibilidad

Una versión es visible para un snapshot cuando:

```text
version.created_at <= snapshot.read_at
y
(version.deleted_at no existe o snapshot.read_at < version.deleted_at)
```

La igualdad con `deleted_at` deja de ser visible. Esto permite que una
actualización cerrada en `t12` y una versión nueva creada en `t12` produzcan
una frontera clara: snapshots anteriores a `t12` ven la versión vieja;
snapshots en `t12` o posteriores ven la nueva versión si existe.

## Decisión de diseño

`Snapshot` todavía no conoce transacciones activas, confirmadas o abortadas. Es
un timestamp lógico simple porque este paso enseña la mecánica de elegir una
versión, no el protocolo completo de aislamiento.

La regla queda en el módulo de MVCC y se prueba por separado para que el
siguiente issue pueda profundizar en visibilidad por timestamp lógico sin
redefinir la representación base.
