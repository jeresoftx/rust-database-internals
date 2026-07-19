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

La siguiente línea natural es cerrar la verificación final del capítulo y
preparar el paso hacia LSM Tree, porque B-Tree ya dejó el vocabulario base de
orden, altura, separadores, punteros y costo local.

El checklist detallado vive en
[`docs/superpowers/plans/2026-07-18-rust-database-internals-course.md`](docs/superpowers/plans/2026-07-18-rust-database-internals-course.md).

## Capítulos Planeados

| # | Capítulo | Estado |
|---|----------|--------|
| 01 | B-Tree | benchmarked |
| 02 | LSM Tree | planned |
| 03 | Índices | planned |
| 04 | Transacciones | planned |
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
