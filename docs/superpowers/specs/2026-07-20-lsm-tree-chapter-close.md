# Especificación: cierre de capítulo LSM Tree

> **Issue:** #106
> **Milestone:** 02 LSM Tree
> **Estado:** benchmarked.

## Propósito

Este paso cierra el capítulo 02 de LSM Tree con la anatomía educativa esperada:
ejemplos progresivos, ejercicios, soluciones, diagrama y benchmark manual.

## Alcance actual

- `docs/02-lsm-tree.md` se eleva a `benchmarked`.
- Se agregan ejemplos progresivos:
  - `lsm_tree_basic`;
  - `lsm_tree_intermediate`;
  - `lsm_tree_advanced`.
- Se agregan soluciones ejecutables:
  - `lsm_tree_flush`;
  - `lsm_tree_precedence`;
  - `lsm_tree_compaction`.
- Se agrega `diagrams/02-lsm-tree.mmd`.
- Se agrega `benches/lsm_tree_bench.rs`.
- README, ROADMAP y checklist general quedan alineados con el estado
  `benchmarked`.

## Decisión de diseño

No se agregan tombstones, bloom filters, niveles, archivos reales ni políticas
avanzadas de compaction. Este cierre conserva el modelo educativo actual y lo
vuelve enseñable como capítulo completo, sin convertirlo en un motor LSM de
producción.
