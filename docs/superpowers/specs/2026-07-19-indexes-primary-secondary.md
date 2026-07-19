# Especificación: Índices Primarios Y Secundarios

> **Issue:** #23  
> **Milestone:** 03 Índices  
> **Estado:** borrador técnico en evolución.

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
- `IndexUniqueness`: `Unique` o `NonUnique`.
- `IndexTarget`: `RecordPointer` o `PrimaryKey(ColumnName)`.
- `IndexEntryKey`: llave almacenada dentro de un índice.
- `PrimaryKeyValue`: valor de primary key referenciado por el índice.
- `IndexEntries`: colección educativa de entradas con regla de unicidad.
- `Selectivity`: llaves distintas, filas indexadas y proporción educativa.
- `SelectivityClass`: `Empty`, `High`, `Medium` o `Low`.

## Invariantes

- Un nombre de índice en blanco devuelve `IndexError::BlankIndexName`.
- Un nombre de columna en blanco devuelve `IndexError::BlankColumnName`.
- `IndexDefinition::primary` produce rol `Primary`.
- `IndexDefinition::primary` produce unicidad `Unique`.
- `IndexDefinition::primary` resuelve hacia `IndexTarget::RecordPointer`.
- `IndexDefinition::secondary` produce rol `Secondary`.
- `IndexDefinition::secondary` produce unicidad `NonUnique`.
- `IndexDefinition::secondary` resuelve hacia
  `IndexTarget::PrimaryKey(primary_key_column)`.
- `IndexDefinition::unique_secondary` produce rol `Secondary`.
- `IndexDefinition::unique_secondary` produce unicidad `Unique`.
- `IndexDefinition::unique_secondary` resuelve hacia
  `IndexTarget::PrimaryKey(primary_key_column)`.
- `IndexEntries` con `IndexUniqueness::Unique` rechaza duplicados con
  `IndexError::DuplicateIndexKey`.
- `IndexEntries` con `IndexUniqueness::NonUnique` permite varias primary keys
  para una misma llave.
- Buscar una llave ausente devuelve una lista vacía.
- `IndexEntries::selectivity` calcula llaves distintas entre filas indexadas.
- Un índice vacío tiene selectividad `0.0` y clase `Empty`.
- Un índice con proporción mayor o igual a `0.8` se clasifica como `High`.
- Un índice con proporción mayor o igual a `0.3` y menor que `0.8` se clasifica
  como `Medium`.
- Un índice no vacío con proporción menor que `0.3` se clasifica como `Low`.
- `estimated_candidates_for` devuelve el número de primary keys asociadas a la
  llave buscada.

## Decisión De Diseño

El índice secundario apunta a la primary key y no directamente a
`RecordPointer`. Esto enseña una idea importante: las rutas alternativas de
búsqueda no deberían inventar una segunda identidad de fila. Primero encuentran
la identidad canónica; después esa identidad permite resolver la ubicación.

El issue #24 agrega la primera regla de duplicados. El issue #25 agrega
selectividad y estimación de candidatos. La implementación todavía no modela
costo de mantenimiento; esa pieza queda para el issue #26.
