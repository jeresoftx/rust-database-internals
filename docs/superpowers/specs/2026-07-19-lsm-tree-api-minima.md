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
paso. El issue #18 agregó escrituras en memoria. El issue #19 agregó flush de
MemTable a SSTable. El issue #20 agrega búsqueda con precedencia explícita.
El issue #21 agrega compaction educativa entre segmentos existentes. El issue
#22 documenta los tradeoffs frente a B-Tree como primera lectura comparativa
del capítulo.

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
- Una búsqueda revisa primero la `MemTable`.
- Si la clave no está en memoria, la búsqueda revisa las `SSTable` desde la más
  reciente hasta la más antigua.
- Un `CompactionPlan` necesita al menos un segmento de entrada.
- Un `CompactionPlan` no acepta entradas duplicadas.
- El segmento de salida de una compaction debe ser nuevo.
- Una compaction rechaza entradas que no existen en el árbol.
- Una compaction rechaza una salida que ya existe como segmento del árbol.
- Una compaction conserva los segmentos que no participan en el plan.
- Una compaction descarta versiones viejas de una clave cuando otro segmento
  de entrada contiene una versión más reciente.
- Una compaction produce una nueva `SSTable` ordenada por clave y la agrega
  como el segmento más reciente.

## Semántica De Compaction

La compaction educativa modela una decisión central de las LSM Tree: muchas
escrituras rápidas generan varios snapshots inmutables, y después el sistema
reduce deuda de lectura fusionando segmentos.

Durante el merge:

- Se leen solo los segmentos indicados por `CompactionPlan`.
- Para claves únicas, el par clave/valor se copia al segmento de salida.
- Para claves repetidas, se conserva el valor del segmento más reciente dentro
  del orden de creación del árbol.
- Las versiones anteriores de esa misma clave se descartan porque ya no son la
  versión visible para una búsqueda normal.
- Los segmentos fuera del plan no se tocan, porque pertenecen a otra frontera
  de compactación.

Esta versión todavía no modela tombstones, niveles, tamaños de página ni
políticas de selección de segmentos. Esas piezas pertenecen a pasos posteriores
del capítulo; aquí la meta es fijar la invariante visible de representación.

## Relación Con B-Tree

B-Tree enseña orden local dentro de nodos y crecimiento por split. LSM Tree
debe enseñar otro eje: aceptar escrituras rápido en memoria, congelar segmentos
ordenados e ir reparando el costo de lectura mediante compaction.

Esta primera API existe para que esa comparación sea explícita desde el diseño,
no una coincidencia de implementación.
