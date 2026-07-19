# Rust Database Internals

Repositorio del camino troncal de Jeresoft Academy para estudiar internals de
bases de datos en Rust. Pertenece al Semestre 3 del plan de estudios junto con
`rust-concurrency` (RFC-0001 §10).

El objetivo no es envolver PostgreSQL, MySQL, MongoDB ni otro motor real. El
objetivo es construir modelos educativos pequeños para entender cómo piensa un
motor de base de datos por dentro: cómo representa datos, cómo indexa, cómo
mantiene transacciones, cómo escribe logs, cómo se recupera y cómo decide un
plan de consulta.

## Qué Contiene

- Capítulos en Markdown compatibles con mdBook.
- Modelos Rust idiomáticos, un mecanismo por módulo.
- Ejemplos progresivos: básico, intermedio, avanzado y caso real.
- Tests unitarios, tests de integración y doctests.
- Benchmarks que comparan análisis teórico con mediciones.
- Diagramas Mermaid y recursos visuales.
- Ejercicios graduados con soluciones para niveles 1 a 3.

## Lugar En El Camino

Este curso vive en el Semestre 3. Recibe ideas de `rust-data-structures`,
`rust-algorithms`, `rust-operating-systems` y `rust-concurrency`. Alimenta
`rust-distributed-systems`, `rust-system-design`, `rust-performance` y los
proyectos integradores como Redis, SQLite y Kafka.

`rust-database-internals` es canónico para mecanismos internos de motores:
B-Tree, LSM Tree, índices, transacciones, ACID, MVCC, WAL, recovery,
replicación y query optimizer. Los motores reales y comparaciones prácticas
pertenecen a la secuela propuesta `rust-database-systems`.

## Capítulos

| # | Capítulo | Módulo | Estado |
|---|----------|--------|--------|
| 01 | B-Tree | `src/btree.rs` | benchmarked |
| 02 | LSM Tree | `src/lsm_tree.rs` | draft |
| 03 | Índices | `src/indexes.rs` | benchmarked |
| 04 | Transacciones | `src/transactions.rs` | benchmarked |
| 05 | ACID | `src/acid.rs` | benchmarked |
| 06 | MVCC | `src/mvcc.rs` | tested |
| 07 | Write-Ahead Log | `src/wal.rs` | planned |
| 08 | Recovery | `src/recovery.rs` | planned |
| 09 | Replicación | `src/replication.rs` | planned |
| 10 | Query Optimizer | `src/query_optimizer.rs` | planned |

Estados posibles: `planned`, `draft`, `implemented`, `tested`,
`benchmarked`, `reviewed`, `published`.

## Frontera Con Motores Reales

PostgreSQL, MySQL, MongoDB, Neo4j, motores de búsqueda y motores vectoriales
son importantes, pero no son dependencias obligatorias de este curso. Aquí se
implementan modelos propios en Rust para estudiar los mecanismos internos.

La comparación con motores reales se documenta como secuela en
`rust-database-systems`. Los laboratorios reproducibles con Docker pertenecen
al curso de DevOps o a una posible unidad futura `rust-docker`.

## Estructura Esperada

```text
AGENTS.md
ROADMAP.md
LICENSE.md
LICENSE-MIT
LICENSE-APACHE
LICENSE-CC-BY-SA-4.0.md
docs/
  SUMMARY.md
src/
  lib.rs
examples/
  soluciones/
tests/
benches/
diagrams/
assets/
```

## Cómo Usarlo

Ejecutar tests:

```bash
cargo test
```

Formatear:

```bash
cargo fmt
```

Verificación completa:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
```

Ejecutar benchmarks:

```bash
cargo bench --bench btree_bench
cargo bench --bench indexes_bench
cargo bench --bench transactions_bench
cargo bench --bench acid_bench
```

## Gobernanza

- `AGENTS.md` es la guía de arranque para humanos e IA en este repositorio.
- `ROADMAP.md` registra el avance del curso sin convertirlo en una fecha límite.
- `docs/superpowers/plans/2026-07-18-rust-database-internals-course.md`
  contiene el checklist de implementación inicial.
- `LICENSE.md` resume la doble licencia: código bajo `MIT OR Apache-2.0`;
  contenido educativo bajo `CC BY-SA 4.0`.

## Filosofía

Este repositorio debe poder leerse como un libro de ingeniería. La claridad
gana sobre el ingenio, la calidad gana sobre la velocidad, y ningún capítulo se
considera publicable hasta cumplir la anatomía completa de RFC-0001 §14.
