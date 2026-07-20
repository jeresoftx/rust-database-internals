# MVCC: cierre pedagógico del capítulo

> **Issue:** #110  
> **Milestone:** 06 MVCC  
> **Estado:** benchmarked

## Contexto

MVCC ya contaba con modelo Rust, pruebas, ejemplos iniciales y documentación en
estado `tested`. El capítulo todavía necesitaba el cierre que vuelve explícita
la anatomía de Jeresoft Academy para un bloque `benchmarked`: ejercicios,
soluciones ejecutables, diagrama fuente y benchmark manual.

## Decisión

Se eleva MVCC a `benchmarked` sin marcarlo como `reviewed` ni `published`.
El cierre conserva el modelo actual: cadenas de versiones, snapshots por
timestamp lógico, borrado lógico y decisiones explícitas de visibilidad.

## Alcance

- Registrar soluciones ejecutables para ejercicios graduados.
- Agregar `diagrams/06-mvcc.mmd` como fuente Mermaid del capítulo.
- Agregar `benches/mvcc_bench.rs` para medir operaciones educativas.
- Actualizar README, ROADMAP y plan para reflejar el nuevo estado.

## Fuera de Alcance

- No se agregan transacciones activas, confirmadas o abortadas.
- No se implementa vacuum.
- No se modelan XIDs reales de PostgreSQL.
- No se integran índices, heap pages ni WAL.
- No se marca el capítulo como revisado o publicado.

## Verificación Esperada

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets`
- `cargo run --example mvcc_basic`
- `cargo run --example mvcc_intermediate`
- `cargo run --example mvcc_advanced`
- `cargo run --example mvcc_visibility`
- `cargo run --example mvcc_snapshot_read`
- `cargo run --example mvcc_visibility_window`
- `cargo run --example mvcc_logical_delete`
- `cargo test --doc`
- `cargo bench --bench mvcc_bench`
- `git diff --check`
