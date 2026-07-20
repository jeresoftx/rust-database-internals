# Índices: corrección de trazabilidad del checklist

> **Issue:** #108  
> **Milestone:** 03 Índices  
> **Estado:** documentado

## Contexto

El capítulo 03, Índices, ya estaba representado como `benchmarked` en
`README.md` y `ROADMAP.md`. Además, la sección específica de la Task 3 en el
plan del curso ya tenía completados sus puntos internos: diseño de índices
primarios y secundarios, unicidad, no unicidad, selectividad y costos de
mantenimiento.

El único desfase era documental: el checklist general del plan todavía mostraba
pendiente `Desarrollar capítulo 03: Índices`.

## Decisión

Se marca como completado el renglón general de Índices en el checklist del plan.
Este ajuste no cambia el código del crate ni amplía el alcance pedagógico del
capítulo.

## Alcance

- Corregir el checklist general del plan versionado.
- Registrar la razón del cambio para conservar trazabilidad entre issue,
  milestone, checklist y PR.
- Mantener el estado del capítulo como `benchmarked`.

## Fuera de Alcance

- No se marca el capítulo como `reviewed`.
- No se marca el capítulo como `published`.
- No se agregan nuevos modelos, ejemplos, ejercicios ni benchmarks.
- No se modifica código de producción.

## Verificación Esperada

Este cambio es documental. La verificación mínima esperada es:

- `git diff --check`
- revisión del estado limpio del árbol de trabajo antes de crear el PR.
