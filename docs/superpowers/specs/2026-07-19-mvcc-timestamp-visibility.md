# Especificación: visibilidad por timestamp lógico MVCC

> **Issue:** #36
> **Milestone:** 06 MVCC
> **Estado:** draft.

## Propósito

Este paso vuelve explícita la regla de visibilidad por timestamp lógico. El
modelo ya podía leer por `Snapshot`; ahora también explica por qué una versión
es visible o invisible en un timestamp dado.

## Alcance actual

- `VisibilityDecision` modela tres resultados: `Visible`, `NotYetCreated` y
  `Deleted`.
- `RecordVersion::visibility_at` evalúa una versión contra un
  `LogicalTimestamp`.
- `RecordVersion::is_visible_at` ofrece el booleano derivado de esa decisión.
- `RecordVersion::is_visible_in` usa la misma regla a través de `Snapshot`.
- `VersionChain::read_at` permite leer directamente por timestamp lógico.
- `tests/mvcc_test.rs` cubre los bordes antes de creación, en creación, justo
  antes de borrado y en el timestamp de borrado.
- `examples/mvcc_visibility.rs` muestra la ventana visible `[created_at,
  deleted_at)`.
- `docs/06-mvcc.md` documenta las decisiones de visibilidad.
- El checklist del plan marca completada la visibilidad por timestamp lógico.

## Regla

Para una versión con `created_at = t10` y `deleted_at = t12`:

```text
t09 -> NotYetCreated
t10 -> Visible
t11 -> Visible
t12 -> Deleted
```

La ventana visible es cerrada por la izquierda y abierta por la derecha:
`[created_at, deleted_at)`. Si `deleted_at` no existe, la versión permanece
visible desde `created_at` en adelante.

## Decisión de diseño

El modelo separa decisión explicativa de booleano operativo. `VisibilityDecision`
sirve para documentación, tests y casos de borde; `is_visible_at` e
`is_visible_in` mantienen una API cómoda para lecturas.

La comparación con PostgreSQL se deja para el issue siguiente para no mezclar
la regla educativa con detalles como `xmin`, `xmax`, snapshots de transacción,
tuplas muertas o vacuum.
