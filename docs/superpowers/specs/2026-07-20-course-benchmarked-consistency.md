# Curso: consistencia final de estado benchmarked

> **Issue:** #118  
> **Milestone:** 00 Gobernanza y planificación  
> **Estado:** documentado

## Contexto

Después del cierre de Replicación, los diez capítulos del curso aparecen como
`benchmarked` en `README.md` y `ROADMAP.md`. La pasada final detectó señales
documentales rezagadas: B-Tree conservaba un estado antiguo dentro del capítulo
y varios textos de "siguiente paso natural" apuntaban a tareas que ya quedaron
cerradas.

## Decisión

Se corrige la consistencia documental del curso sin cambiar código de
producción ni elevar ningún capítulo a `reviewed` o `published`.

## Alcance

- Alinear el estado visible de B-Tree con README y ROADMAP.
- Reemplazar siguientes pasos obsoletos por una indicación de revisión humana
  del bloque `benchmarked`.
- Registrar el cierre de consistencia con issue, milestone y límites claros.

## Fuera de Alcance

- No se cambia ninguna API Rust.
- No se agregan ejemplos, ejercicios ni benchmarks.
- No se marca ningún capítulo como revisado o publicado.
- No se modifica la gobernanza del repositorio.

## Verificación Esperada

- `rg -n "^> \\*\\*Estado:\\*\\*" docs/*.md`
- `rg -n "borrador técnico|El siguiente capítulo natural|volver al capítulo|corregir la marca pendiente|cerrar Write-Ahead Log" docs/*.md`
- `git diff --check`
