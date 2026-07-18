# AGENTS.md

Este repositorio es parte de la colección camino troncal / Semestre 3 de
Jeresoft Academy y se rige por la RFC-0001 (manual fundacional).

## Objetivo

Crear el mejor recurso educativo posible sobre internals de bases de datos en
Rust.

Todo cambio debe mejorar simultáneamente:

- calidad técnica
- claridad
- documentación
- mantenibilidad

## Antes de escribir código

Siempre, en este orden (RFC-0001 §13):

1. Explicar el concepto.
2. Explicar el problema.
3. Comparar alternativas.
4. Justificar la implementación.

## Código

Conforme a RFC-0001 §13:

- Rust idiomático.
- Clippy limpio y rustfmt sin diffs.
- Sin `unsafe` salvo justificación documentada con comentario `// SAFETY:` y
  revisión humana explícita.
- Comentarios donde aporten valor.
- Los modelos de mecanismos internos deben declarar invariantes, límites,
  costos, modos de falla y relación con motores reales.
- No se agrega una dependencia externa sin justificar por qué el capítulo no
  puede enseñar el concepto con la biblioteca estándar o con un modelo pequeño.

## Documentación

Todo capítulo sigue las doce secciones de RFC-0001 §14 y la plantilla de §16.
Toda nueva funcionalidad incluye:

- README actualizado.
- Diagramas Mermaid (RFC-0001 §12).
- Ejemplos ejecutables.
- Tests.
- Benchmarks cuando apliquen; si no aplican, se declara.

## Frontera Del Curso

Este curso no depende de PostgreSQL, MySQL, MongoDB ni motores reales. Esos
motores pueden citarse para comparar decisiones, pero el canon de este repo son
modelos educativos propios en Rust.

La secuela práctica propuesta `rust-database-systems` cubre motores reales,
geoespacial, búsquedas fonéticas, búsqueda contextual y laboratorios Docker.

## Nunca

- Agregar dependencias innecesarias.
- Optimizar prematuramente.
- Duplicar código.
- Omitir documentación.
- Publicar capítulos parciales.
- Presentar bases de datos como magia: cada mecanismo debe declarar qué
  garantiza, qué no garantiza y qué falla cuando los recursos son finitos.
- Reexplicar Docker, DevOps o motores reales como canon interno del curso.

## Filosofía

Este repositorio debe poder utilizarse como un libro de ingeniería. Nunca
sacrificar claridad por ingenio. Explicar el porqué, no solo el cómo.
