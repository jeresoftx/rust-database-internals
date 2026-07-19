# LSM Tree

> **Estado:** borrador técnico de representación y tradeoffs.
> **Alcance actual:** `MemTable`, `SSTable`, `SegmentId`,
> `CompactionPlan`, escritura en memoria, flush, búsqueda por precedencia,
> compaction educativa y comparación contra B-Tree.

## Por Qué Existe

Una LSM Tree existe porque no todos los índices deben optimizar primero la
lectura puntual. Hay cargas donde el costo dominante es escribir muchísimo, de
forma sostenida, sin convertir cada inserción en una actualización aleatoria de
páginas persistentes.

La idea central es aceptar escrituras recientes en memoria, congelarlas en
segmentos ordenados e inmutables y compactar esos segmentos después. El sistema
cambia el costo inmediato de escribir por un costo diferido de mantenimiento.
Ese intercambio es el corazón del capítulo.

B-Tree enseña cómo mantener una estructura ordenada y balanceada mientras se
inserta. LSM Tree enseña otra pregunta: qué pasa si primero escribimos rápido,
dejamos evidencia inmutable y reparamos la forma del índice por lotes.

## Modelo Actual Del Curso

El modelo Rust actual representa una LSM Tree educativa con cuatro piezas:

- `MemTable`: tabla mutable en memoria donde aterrizan las escrituras
  recientes;
- `SSTable`: segmento ordenado e inmutable producido por flush o compaction;
- `SegmentId`: identidad lógica de un segmento;
- `CompactionPlan`: intención explícita de fusionar segmentos y producir uno
  nuevo.

La búsqueda revisa primero la `MemTable` y después recorre los segmentos desde
el más reciente hasta el más antiguo. Esa precedencia importa porque una misma
clave puede aparecer en varios lugares con versiones distintas.

La compaction educativa fusiona segmentos existentes, conserva la versión más
reciente de cada clave y descarta versiones viejas. El resultado es una nueva
`SSTable` ordenada por clave. Los segmentos que no participan en el plan se
mantienen intactos.

No es todavía una LSM Tree de producción. Tombstones, niveles, bloom filters,
índices por bloque, tamaños de página, archivos reales, WAL, concurrencia,
políticas de compaction y recovery quedan para pasos posteriores.

## Diagrama Mental

```mermaid
flowchart LR
    write["write(key, value)"] --> memtable["MemTable\nmutable, ordenada"]
    memtable -->|"flush"| s1["SSTable 1\ninmutable"]
    memtable -->|"flush"| s2["SSTable 2\ninmutable"]
    memtable -->|"flush"| s3["SSTable 3\ninmutable"]

    search["search(key)"] --> memtable
    search -->|"si no está en memoria"| newest["segmentos\nmás reciente a más antiguo"]
    newest --> s3
    newest --> s2
    newest --> s1

    s1 --> compaction["CompactionPlan"]
    s2 --> compaction
    compaction --> merged["SSTable nueva\nversiones visibles"]
```

## Costos Y Decisiones Frente A B-Tree

| Decisión | B-Tree | LSM Tree |
|----------|--------|----------|
| Escritura | Inserta dentro de la estructura principal. | Escribe primero en memoria y difiere el orden global. |
| Lectura puntual | Busca por altura balanceada y pocas ramas. | Revisa MemTable y puede revisar varios segmentos. |
| Orden | Se mantiene durante cada operación. | Se mantiene por segmento; el orden global se recompone por compaction. |
| Mutabilidad | Actualiza nodos existentes. | Produce segmentos inmutables y reemplaza por lotes. |
| Costo diferido | Split y rebalanceo cuando un nodo se llena. | Flush y compaction cuando se acumulan segmentos. |
| Amplificación | Puede pagar escrituras aleatorias. | Puede pagar reescrituras durante compaction. |
| Simplicidad de lectura | La ruta de lectura es directa. | La ruta de lectura necesita precedencia entre versiones. |
| Simplicidad de escritura | La inserción toca la estructura ordenada. | La escritura reciente es barata mientras cabe en MemTable. |

La comparación no dice que una estructura sea "mejor" de forma absoluta. Dice
qué costo decide pagar cada una y en qué momento.

## Costo De Escritura

En un B-Tree, insertar significa ubicar la hoja correcta, modificarla y partir
nodos cuando se llenan. El beneficio es que la estructura queda lista para
buscar con una ruta balanceada.

En una LSM Tree, escribir en la `MemTable` es barato porque ocurre en memoria y
usa una estructura ordenada local. Cuando la `MemTable` se llena, el sistema
hace flush y produce una `SSTable` inmutable. Esa decisión favorece cargas con
muchas escrituras consecutivas.

El costo no desaparece. Se mueve. Más tarde, compaction tendrá que leer varios
segmentos, fusionarlos y escribir un segmento nuevo. Ese trabajo de fondo es el
precio de haber aceptado escrituras rápidas al inicio.

## Costo De Lectura

En un B-Tree, una búsqueda baja por niveles hasta una hoja. La altura crece
lento porque cada nodo agrupa muchas claves. La lectura puntual suele tener una
ruta clara: nodo raíz, nodos internos, hoja.

En una LSM Tree, una búsqueda debe respetar versiones. Primero revisa memoria,
luego segmentos recientes y después segmentos antiguos. Si la clave aparece en
varios segmentos, gana la versión más reciente.

Sin ayudas adicionales, demasiados segmentos elevan el costo de lectura. Por
eso las LSM Tree reales combinan compaction, bloom filters, índices por bloque y
cachés. El modelo educativo empieza sin esas optimizaciones para que la
precedencia sea visible.

## Compaction Como Mantenimiento

Compaction no es una limpieza cosmética. Es la operación que reduce deuda
estructural:

- junta segmentos que antes estaban separados;
- elimina versiones viejas que ya no son visibles;
- mantiene el orden por clave en el nuevo segmento;
- reduce la cantidad de lugares que una búsqueda debe consultar;
- prepara el terreno para políticas por niveles o tamaños.

En este repo, `CompactionPlan` es explícito porque el curso quiere enseñar que
la compaction es una decisión del sistema, no un efecto mágico de escribir. El
plan dice qué segmentos se leen y cuál será el segmento de salida.

## Invariantes Del Modelo

- Una `MemTable` necesita capacidad positiva.
- Una escritura nueva consume una entrada de la `MemTable`.
- Reescribir una clave existente no aumenta `len`.
- Un flush vacío falla.
- Un flush con datos crea una `SSTable` ordenada y vacía la `MemTable`.
- Una búsqueda revisa memoria antes que segmentos.
- Entre segmentos, una búsqueda revisa de más reciente a más antiguo.
- Una compaction necesita entradas existentes.
- Una compaction no puede reutilizar un segmento existente como salida.
- Una compaction conserva la versión más reciente de cada clave compactada.
- Una compaction no modifica segmentos fuera del plan.

Estas reglas están escritas para que el lector pueda comparar representación,
operaciones y errores sin saltar todavía a archivos reales.

## Cuándo Preferir Cada Modelo

Un B-Tree suele ser una buena intuición inicial cuando:

- las lecturas puntuales son frecuentes;
- las escrituras deben dejar la estructura principal lista de inmediato;
- importa mantener una sola ruta clara hacia cada clave;
- el costo de actualización en sitio es aceptable.

Una LSM Tree suele ser una buena intuición inicial cuando:

- hay muchas escrituras sostenidas;
- conviene convertir escrituras aleatorias en escrituras por lotes;
- se acepta hacer mantenimiento de fondo;
- el sistema puede compensar lecturas con filtros, cachés y compaction.

Estas son intuiciones de diseño, no recetas. Un motor real mezcla muchas capas:
WAL, buffer pool, compresión, concurrencia, recovery, estadísticas y decisiones
de almacenamiento físico.

## Relación Con El Resto Del Curso

LSM Tree conecta con capítulos posteriores:

- índices, porque una SSTable necesita estructuras auxiliares para buscar
  rápido;
- transacciones y ACID, porque las versiones visibles deben respetar reglas de
  consistencia;
- MVCC, porque conservar o descartar versiones deja de ser trivial cuando hay
  lectores concurrentes;
- WAL y recovery, porque aceptar escrituras en memoria exige poder recuperarlas;
- query optimizer, porque el costo de leer varios segmentos cambia la estimación
  de planes.

El propósito de este capítulo no es copiar RocksDB ni LevelDB. El propósito es
construir una maqueta pequeña y honesta para entender por qué esos motores
existen y qué problema decidieron pagar.
