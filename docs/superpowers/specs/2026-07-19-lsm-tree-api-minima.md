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

Este paso no implementa escritura, búsqueda, flush ni compaction real. Solo
define contratos pequeños que permitan escribir los siguientes tests sin
reinventar vocabulario en cada issue.

## Invariantes Iniciales

- Una `MemTable` necesita capacidad positiva.
- Una `SSTable` se identifica por `SegmentId`.
- Una `SSTable` puede estar vacía como metadato educativo.
- Un `CompactionPlan` necesita al menos un segmento de entrada.
- Un `CompactionPlan` no acepta entradas duplicadas.
- El segmento de salida de una compaction debe ser nuevo.

## Relación Con B-Tree

B-Tree enseña orden local dentro de nodos y crecimiento por split. LSM Tree
debe enseñar otro eje: aceptar escrituras rápido en memoria, congelar segmentos
ordenados e ir reparando el costo de lectura mediante compaction.

Esta primera API existe para que esa comparación sea explícita desde el diseño,
no una coincidencia de implementación.
