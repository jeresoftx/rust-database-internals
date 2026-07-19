# Spec: API Mínima de LSM Tree

> **Issue:** #17  
> **Milestone:** 02 LSM Tree  
> **Estado:** borrador técnico inicial.

## Propósito

La API mínima de LSM Tree nombra las piezas antes de modelar operaciones. El
capítulo debe separar cuatro ideas:

- `MemTable`: estado mutable en memoria para escrituras recientes;
- `SSTable`: segmento ordenado e inmutable producido por flush o compaction;
- `SegmentId`: identidad lógica de un segmento;
- `CompactionPlan`: intención explícita de leer segmentos y producir uno nuevo.

## Frontera Actual

El issue #17 definió contratos pequeños para no reinventar vocabulario en cada
paso. El issue #18 agregó escrituras en memoria. El issue #19 agrega flush de
MemTable a SSTable. Búsqueda entre segmentos y compaction real quedan fuera de
esta frontera.

## Invariantes Iniciales

- Una `MemTable` necesita capacidad positiva.
- Una escritura nueva consume una posición de la `MemTable`.
- Reescribir una clave existente reemplaza su valor sin crecer `len`.
- Las entradas visibles de una `MemTable` se exponen en orden ascendente por
  clave.
- Una `MemTable` llena rechaza claves nuevas hasta que exista flush.
- Un flush vacío devuelve `LsmTreeError::EmptyMemTableFlush`.
- Un flush con datos produce una `SSTable` con snapshot ordenado y deja la
  `MemTable` vacía.
- Una `SSTable` se identifica por `SegmentId`.
- Una `SSTable` puede estar vacía como metadato educativo.
- Una `SSTable` creada por flush no cambia si después se vuelve a escribir en
  la `MemTable`.
- Un `CompactionPlan` necesita al menos un segmento de entrada.
- Un `CompactionPlan` no acepta entradas duplicadas.
- El segmento de salida de una compaction debe ser nuevo.

## Relación Con B-Tree

B-Tree enseña orden local dentro de nodos y crecimiento por split. LSM Tree
debe enseñar otro eje: aceptar escrituras rápido en memoria, congelar segmentos
ordenados e ir reparando el costo de lectura mediante compaction.

Esta primera API existe para que esa comparación sea explícita desde el diseño,
no una coincidencia de implementación.
