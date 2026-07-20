# Write-Ahead Log: cierre pedagógico del capítulo

> **Issue:** #112  
> **Milestone:** 07 Write-Ahead Log  
> **Estado:** benchmarked

## Contexto

Write-Ahead Log ya contaba con representación Rust, pruebas, documentación y
ejemplos iniciales en estado `tested`. El cierre faltante era la anatomía
pedagógica completa para el estado `benchmarked`: ejercicios, soluciones
ejecutables, diagrama fuente y benchmark manual.

## Decisión

Se eleva Write-Ahead Log a `benchmarked` sin marcarlo como `reviewed` ni
`published`. El cierre conserva el alcance educativo actual: registros WAL,
LSN monótono, append-only log, redo, undo y la regla de escribir el log antes
de modificar la página.

## Alcance

- Agregar un ejemplo ejecutable de la regla WAL.
- Registrar soluciones ejecutables para ejercicios graduados.
- Agregar `diagrams/07-write-ahead-log.mmd` como fuente Mermaid.
- Agregar `benches/wal_bench.rs` para medir operaciones educativas.
- Actualizar README, ROADMAP y plan para reflejar el nuevo estado.

## Fuera de Alcance

- No se implementa `fsync`.
- No se modela buffer pool real ni páginas sucias.
- No se implementa recovery en este capítulo.
- No se agregan checkpoints ni compaction de log.
- No se marca el capítulo como revisado o publicado.

## Verificación Esperada

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets`
- `cargo run --example wal_append_only`
- `cargo run --example wal_redo_undo`
- `cargo run --example wal_rule`
- `cargo run --example wal_append_order`
- `cargo run --example wal_redo_after`
- `cargo run --example wal_rule_before_page`
- `cargo test --doc`
- `cargo bench --bench wal_bench`
- `git diff --check`
