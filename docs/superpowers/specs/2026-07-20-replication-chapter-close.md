# Replicación: cierre pedagógico del capítulo

> **Issue:** #116  
> **Milestone:** 09 Replicación  
> **Estado:** benchmarked

## Contexto

Replicación ya contaba con modelo Rust, pruebas, documentación y ejemplos
iniciales en estado `tested`. El cierre faltante era completar la anatomía para
el estado `benchmarked`: ejercicios, soluciones ejecutables, diagrama fuente y
benchmark manual.

## Decisión

Se eleva Replicación a `benchmarked` sin marcarla como `reviewed` ni
`published`. El cierre conserva el alcance educativo actual: primary/replica,
copia ordenada del WAL, lag observable y confirmación asíncrona o síncrona.

## Alcance

- Registrar soluciones ejecutables para ejercicios graduados.
- Agregar `diagrams/09-replicacion.mmd` como fuente Mermaid.
- Agregar `benches/replication_bench.rs` para medir operaciones educativas.
- Actualizar README, ROADMAP y plan para reflejar el nuevo estado.

## Fuera de Alcance

- No se implementa failover.
- No se implementa quorum ni consenso.
- No se modelan lecturas reales desde primary o réplica.
- No se marca el capítulo como revisado o publicado.

## Verificación Esperada

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets`
- `cargo run --example replication_primary_replica`
- `cargo run --example replication_lag`
- `cargo run --example replication_ack_modes`
- `cargo run --example replication_copy_wal`
- `cargo run --example replication_measure_lag`
- `cargo run --example replication_ack_tradeoff`
- `cargo test --doc`
- `cargo bench --bench replication_bench`
- `git diff --check`
