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
compaction educativa y documentación de tradeoffs frente a B-Tree. También
incluye ejemplos progresivos, ejercicios, soluciones, diagrama Mermaid y
benchmark manual. Su estado visible es `benchmarked`: tiene medición
educativa, pero todavía no se marca como `reviewed` ni `published`.

Índices ya cuenta con representación de índice primario y secundario, reglas de
unicidad, selectividad, documentación de costos, ejemplos progresivos,
ejercicios y benchmark manual. Su estado visible es `benchmarked`: tiene
medición educativa, pero todavía no se marca como `reviewed` ni `published`.

Transacciones ya cuenta con identidad, estado, administrador educativo, ciclo
de vida mínimo mediante `begin`, `commit` y `rollback`, y conflictos simples
con locks exclusivos por recurso lógico. También incluye documentación de
atomicidad y aislamiento, diagrama Mermaid, ejemplos progresivos, ejercicios,
soluciones y benchmark manual. Su estado visible es `benchmarked`: tiene
medición educativa, pero todavía no se marca como `reviewed` ni `published`.

ACID ya cuenta con documentación de Atomicity, Consistency, Isolation y
Durability desde internals, modelos Rust mínimos por propiedad, ejercicios de
fallas parciales, soluciones ejecutables, diagrama Mermaid y benchmark manual.
Su estado visible es `benchmarked`: tiene medición educativa, pero todavía no
se marca como `reviewed` ni `published`.

MVCC ya cuenta con representación inicial de versiones de registro, timestamps
lógicos, metadatos de visibilidad, snapshot reads básicos y decisiones
explícitas de visibilidad por timestamp. También incluye comparación educativa
con PostgreSQL sobre `xmin`, `xmax`, snapshots, aislamiento y vacuum. Su estado
visible es `tested`: el bloque tiene modelo, pruebas, ejemplos y documentación,
pero todavía no se marca como `reviewed` ni `published`.

Write-Ahead Log ya cuenta con representación inicial de registros: LSN,
transacción lógica, página lógica, imágenes `before`/`after` y operaciones
`Begin`, `Update`, `Commit` y `Rollback`. También incluye log append-only en
memoria, redo, undo y la regla central de escribir el log antes de modificar
una página. Su estado visible es `tested`: el bloque tiene modelo, pruebas,
ejemplos y documentación, pero todavía no se marca como `reviewed` ni
`published`.

Recovery ya cuenta con un primer plan educativo para interpretar el WAL después
de una caída: transacciones con cambios y `Commit` se clasifican para redo, y
transacciones con cambios sin `Commit` ni `Rollback` se clasifican para undo.
También incluye replay educativo sobre `PageStore`, con redo en orden de WAL y
undo en orden inverso. El capítulo documenta checkpoints como puntos de partida
para acotar la lectura del WAL durante recovery. Su estado visible es `tested`:
el bloque tiene modelo, pruebas, ejemplos y documentación, pero todavía no se
marca como `reviewed` ni `published`.

Replicación ya cuenta con un modelo inicial primary/replica: el primary acepta
escrituras locales, las réplicas las rechazan y el cluster copia registros del
WAL del primary hacia una réplica. También mide lag como registros pendientes
entre el primary y cada réplica, y modela confirmación asíncrona frente a
confirmación síncrona. El capítulo documenta tradeoffs de consistencia entre
latencia, frescura y carga de lectura al elegir confirmación y fuente de
lectura. Su estado visible es `tested`: el bloque tiene modelo, pruebas,
ejemplos y documentación, pero todavía no se marca como `reviewed` ni
`published`.

Query Optimizer ya cuenta con una primera representación de plan lógico y plan
físico. El plan lógico expresa intención de consulta: relación, selección y
proyección. El plan físico separa forma de ejecución y ruta de acceso; ya puede
nombrar `TableScan` e `IndexScan`, además de `Unchosen` cuando la decisión aún
no se toma. También cuenta con un catálogo mínimo de estadísticas para estimar
costo por filas leídas, filas producidas y unidades de trabajo. El capítulo
documenta por qué `EXPLAIN` existe en motores reales e incluye ejemplos,
ejercicios, soluciones y benchmark manual. Su estado visible es `benchmarked`:
tiene medición educativa, pero todavía no se marca como `reviewed` ni
`published`.

El checklist detallado vive en
[`docs/superpowers/plans/2026-07-18-rust-database-internals-course.md`](docs/superpowers/plans/2026-07-18-rust-database-internals-course.md).

## Capítulos Planeados

| # | Capítulo | Estado |
|---|----------|--------|
| 01 | B-Tree | benchmarked |
| 02 | LSM Tree | benchmarked |
| 03 | Índices | benchmarked |
| 04 | Transacciones | benchmarked |
| 05 | ACID | benchmarked |
| 06 | MVCC | tested |
| 07 | Write-Ahead Log | tested |
| 08 | Recovery | tested |
| 09 | Replicación | tested |
| 10 | Query Optimizer | benchmarked |

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
