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

## Decisión De Diseño

El índice secundario apunta a la primary key y no directamente a
`RecordPointer`. Esto enseña una idea importante: las rutas alternativas de
búsqueda no deberían inventar una segunda identidad de fila. Primero encuentran
la identidad canónica; después esa identidad permite resolver la ubicación.

El issue #24 agrega la primera regla de duplicados. La implementación todavía
no modela selectividad ni costo de mantenimiento. Esas piezas quedan para los
issues #25 y #26.
