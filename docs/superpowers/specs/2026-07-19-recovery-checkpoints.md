# Especificación: checkpoints en Recovery

> **Issue:** #44
> **Milestone:** 08 Recovery
> **Estado:** tested.

## Propósito

Este paso cierra el capítulo de Recovery documentando por qué existen los
checkpoints y qué problema resuelven: evitar que recovery tenga que leer toda
la historia del WAL desde el inicio.

## Alcance actual

- `docs/08-recovery.md` explica checkpoints como puntos de partida para
  recovery.
- El capítulo distingue checkpoint nítido y checkpoint difuso.
- El texto deja claro que un checkpoint no reemplaza al WAL ni elimina redo y
  undo.
- README y ROADMAP elevan Recovery a `tested`.
- El checklist del plan marca completo el capítulo 08.

## Decisión de diseño

No se agrega una API ejecutable de checkpoints en este issue porque el plan
pidió documentarlos. Una estructura `Checkpoint` sería una decisión de diseño
posterior: tendría que modelar dirty page table, transacciones activas, LSNs de
inicio y relación con flush de páginas. Para este cierre, el objetivo es dejar
el concepto correcto y conectado con WAL, redo y undo.
