# Especificación: representación de plan lógico y físico

> **Issue:** #49
> **Milestone:** 10 Query Optimizer
> **Estado:** tested.

## Propósito

Este paso inicia Query Optimizer con el vocabulario mínimo para distinguir la
intención de una consulta de su forma de ejecución.

## Alcance actual

- `src/query_optimizer.rs` agrega nombres validados para relaciones y columnas.
- El módulo modela predicados simples con columna, operador y literal.
- `LogicalPlan` representa `ReadRelation`, `Select` y `Project`.
- `PhysicalPlan` representa `ReadRelation`, `Filter` y `Project`.
- `PhysicalAccessPath::Unchosen` deja explícito que table scan, index scan y
  costo pertenecen a los issues siguientes.
- `tests/query_optimizer_test.rs` cubre la representación lógica, física e
  invariantes de validación.
- `docs/10-query-optimizer.md` documenta la diferencia entre plan lógico y
  plan físico sin marcar el capítulo como completo.

## Decisión de diseño

La representación usa árboles pequeños y explícitos en vez de un parser SQL.
El curso no necesita aceptar SQL real en este punto; necesita mostrar cómo un
motor separa intención, ejecución y decisiones pendientes. Mantener
`PhysicalAccessPath::Unchosen` evita mezclar este issue con table scan, index
scan o estimación de costo.
