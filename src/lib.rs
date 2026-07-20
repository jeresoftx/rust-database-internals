//! Curso de internals de bases de datos en Rust para Jeresoft Academy.
//!
//! Este crate acompaña el curso `rust-database-internals`. Cada módulo futuro
//! representará un mecanismo canónico de motores de bases de datos: árboles de
//! búsqueda, LSM trees, índices, transacciones, MVCC, WAL, recovery,
//! replicación y optimización de consultas.
//!
//! La intención inicial no es competir con PostgreSQL, MySQL o MongoDB. La
//! intención es construir modelos educativos pequeños, verificables y bien
//! documentados para entender por qué esos motores existen y qué problemas
//! resuelven.

pub mod acid;
pub mod btree;
pub mod indexes;
pub mod lsm_tree;
pub mod mvcc;
pub mod recovery;
pub mod transactions;
pub mod wal;

/// Devuelve el nombre canónico del curso.
///
/// # Examples
///
/// ```
/// assert_eq!(
///     rust_database_internals::course_name(),
///     "rust-database-internals"
/// );
/// ```
pub fn course_name() -> &'static str {
    "rust-database-internals"
}
