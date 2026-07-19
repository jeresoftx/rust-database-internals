# ROADMAP

Estado de avance de `rust-database-internals`, repositorio del camino troncal
de Jeresoft Academy para internals de bases de datos en Rust.

No hay fechas límite: este es un proyecto de legado (RFC-0001 §1). Este archivo
orienta el avance, pero no convierte el curso en una carrera por terminar.

## Estado Actual

El capítulo 01, B-Tree, ya cuenta con API mínima, pruebas, split educativo,
documentación de invariantes, diagrama Mermaid, ejemplos progresivos,
ejercicios, soluciones y benchmark manual. Su estado visible es
`benchmarked`: el capítulo tiene medición educativa, pero todavía no se marca
como `reviewed` ni `published` hasta completar la revisión humana del bloque.

LSM Tree ya cuenta con API mínima, escrituras en memoria, flush, búsqueda,
compaction educativa y un primer capítulo de tradeoffs frente a B-Tree. Su
estado visible es `draft`: el mecanismo central existe, pero aún faltan
ejemplos, ejercicios, soluciones, diagramas finales y benchmark educativo antes
de elevarlo a `benchmarked`.

Índices ya cuenta con representación de índice primario y secundario, reglas de
unicidad, selectividad, documentación de costos, ejemplos progresivos,
ejercicios y benchmark manual. Su estado visible es `benchmarked`: tiene
medición educativa, pero todavía no se marca como `reviewed` ni `published`.

Transacciones inicia su estado `draft` con identidad, estado y administrador
educativo. Los siguientes pasos agregan `begin`, `commit`, `rollback`,
conflictos simples y documentación de atomicidad e aislamiento.

El checklist detallado vive en
[`docs/superpowers/plans/2026-07-18-rust-database-internals-course.md`](docs/superpowers/plans/2026-07-18-rust-database-internals-course.md).

## Capítulos Planeados

| # | Capítulo | Estado |
|---|----------|--------|
| 01 | B-Tree | benchmarked |
| 02 | LSM Tree | draft |
| 03 | Índices | benchmarked |
| 04 | Transacciones | draft |
| 05 | ACID | planned |
| 06 | MVCC | planned |
| 07 | Write-Ahead Log | planned |
| 08 | Recovery | planned |
| 09 | Replicación | planned |
| 10 | Query Optimizer | planned |

## Alineación RFC-0001

- Este repositorio sigue la plantilla de repositorio de RFC-0001 §15.
- Cada capítulo debe cumplir la anatomía de RFC-0001 §14.
- Cada ejercicio debe seguir los niveles de RFC-0001 §17.
- El uso de IA se rige por RFC-0001 §20: la IA acelera, el criterio humano
  decide.

## Fuera De Alcance Por Ahora

- Usar PostgreSQL, MySQL, MongoDB, Neo4j o Qdrant como dependencias del core.
- Construir un motor de base de datos de producción.
- Usar `unsafe` sin justificación escrita y revisión humana explícita.
- Agregar dependencias externas para esconder mecanismos que el curso debe
  explicar.
- Reexplicar Docker desde cero: ese canon vive en `rust-devops` o en un curso
  Docker futuro.
- Reexplicar motores reales desde cero: esa secuela vive en la propuesta
  `rust-database-systems`.
