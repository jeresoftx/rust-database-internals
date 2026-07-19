# Especificación: Índices Primarios Y Secundarios

> **Issue:** #23  
> **Milestone:** 03 Índices  
> **Estado:** borrador técnico inicial.

## Propósito

La primera frontera del capítulo de Índices define vocabulario. Antes de hablar
de unicidad, selectividad o costo, el curso necesita separar dos roles:

- índice primario: camino canónico desde primary key hacia ubicación de
  registro;
- índice secundario: camino alternativo desde otra columna hacia la primary key.

## API Actual

- `IndexDefinition`: definición declarativa del índice.
- `IndexName`: nombre lógico no vacío.
- `ColumnName`: nombre de columna no vacío.
- `IndexRole`: `Primary` o `Secondary`.
- `IndexTarget`: `RecordPointer` o `PrimaryKey(ColumnName)`.

## Invariantes

- Un nombre de índice en blanco devuelve `IndexError::BlankIndexName`.
- Un nombre de columna en blanco devuelve `IndexError::BlankColumnName`.
- `IndexDefinition::primary` produce rol `Primary`.
- `IndexDefinition::primary` resuelve hacia `IndexTarget::RecordPointer`.
- `IndexDefinition::secondary` produce rol `Secondary`.
- `IndexDefinition::secondary` resuelve hacia
  `IndexTarget::PrimaryKey(primary_key_column)`.

## Decisión De Diseño

El índice secundario apunta a la primary key y no directamente a
`RecordPointer`. Esto enseña una idea importante: las rutas alternativas de
búsqueda no deberían inventar una segunda identidad de fila. Primero encuentran
la identidad canónica; después esa identidad permite resolver la ubicación.

La implementación todavía no modela entradas, duplicados ni selectividad. Esas
piezas quedan para los issues #24 y #25.
