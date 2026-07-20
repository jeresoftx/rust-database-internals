# Recovery: cierre pedagógico del capítulo

> **Issue:** #114  
> **Milestone:** 08 Recovery  
> **Estado:** benchmarked

## Contexto

Recovery ya contaba con modelo Rust, pruebas, documentación y ejemplos
iniciales en estado `tested`. El cierre faltante era completar la anatomía para
el estado `benchmarked`: ejercicios, soluciones ejecutables, diagrama fuente y
benchmark manual.

## Decisión

Se eleva Recovery a `benchmarked` sin marcarlo como `reviewed` ni `published`.
El cierre conserva el alcance educativo actual: analizar WAL, clasificar
transacciones para redo o undo y reproducir el plan sobre `PageStore`.

## Alcance

- Registrar soluciones ejecutables para ejercicios graduados.
- Agregar `diagrams/08-recovery.mmd` como fuente Mermaid.
- Agregar `benches/recovery_bench.rs` para medir operaciones educativas.
- Actualizar README, ROADMAP y plan para reflejar el nuevo estado.

## Fuera de Alcance

- No se implementa una estructura ejecutable de checkpoints.
- No se separan fases ARIES reales de análisis, redo y undo.
- No se modela disco, `fsync` ni buffer pool.
- No se marca el capítulo como revisado o publicado.

## Verificación Esperada

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets`
- `cargo run --example recovery_crash_commit`
- `cargo run --example recovery_replay_wal`
- `cargo run --example recovery_classify_crash`
- `cargo run --example recovery_replay_redo`
- `cargo run --example recovery_undo_reverse`
- `cargo test --doc`
- `cargo bench --bench recovery_bench`
- `git diff --check`
