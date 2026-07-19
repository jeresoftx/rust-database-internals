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

## Flujo Issue → Commit → PR

Antes de tocar código de curso, el plan completo debe existir como milestones e
issues en GitHub. Ese tablero es el checklist operativo del repositorio.

Cada paso del plan se trabaja con trazabilidad mínima:

1. Crear o reutilizar un issue específico para el paso.
2. Crear una rama corta y descriptiva desde `main`.
3. Hacer exactamente un commit principal para ese paso.
4. Abrir un pull request hacia `main`.
5. Asignar el PR a `jeresoftx`.
6. Asociar el PR al mismo milestone del issue que resuelve.
7. Agregar labels coherentes al issue y al PR.
8. Entregar un resumen breve para revisión humana.
9. Esperar aprobación explícita antes de fusionar el PR.

Reglas operativas:

- Un paso del plan equivale a un issue, un commit y un PR.
- Cada issue debe estar asignado a `jeresoftx`.
- Cada issue debe pertenecer al milestone del capítulo o fase correspondiente.
- Cada PR debe estar asignado a `jeresoftx`.
- Cada PR debe pertenecer al mismo milestone que el issue que resuelve.
- Cada issue y PR deben tener labels suficientes para entender tipo, capítulo o
  fase, y estado de revisión.
- No se empuja trabajo directo a `main`, salvo creación inicial del repositorio
  o correcciones administrativas explícitamente aprobadas.
- El PR debe mencionar el issue que resuelve.
- Si un paso resulta demasiado grande para un solo commit, primero se divide en
  pasos más pequeños y se crean issues separados.
- No se cierra ni se fusiona un PR sin revisión humana.
- Después de fusionar, se actualiza el checklist y se continúa con el siguiente
  issue.
- Si se descubre trabajo no planeado, primero se crea o ajusta el issue; luego
  se implementa.

Labels mínimos:

- Tipo: `tipo: documentación`, `tipo: funcionalidad`, `tipo: prueba`.
- Capítulo o fase: por ejemplo `capítulo: b-tree` o `flujo: issue-pr`.
- Estado: `estado: revisión` cuando el PR queda listo para revisión humana.

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
