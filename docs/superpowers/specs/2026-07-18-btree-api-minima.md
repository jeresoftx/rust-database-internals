# Especificación: API mínima de B-Tree

> **Issue:** #3 — B-Tree: diseñar API mínima
> **Estado:** diseño propuesto para revisión
> **Fecha:** 2026-07-18
> **Base:** RFC-0001 §10, §13, §14, §15 y §20.

## Concepto

Un B-Tree es una estructura de índice balanceada por altura. Su propósito es
reducir lecturas al mantener muchas claves por nodo y pocos niveles entre la
raíz y las hojas.

En un motor real, cada nodo suele representar una página de almacenamiento. En
este curso, el primer modelo será en memoria y educativo: mostrará cómo se
ordenan las claves, cómo se recorren los hijos y cómo se mantiene el balanceo
sin mezclar todavía paginación, WAL, recovery ni concurrencia.

## Problema Que Resuelve Esta API

El capítulo necesita una frontera clara antes de implementar comportamiento. La
API mínima debe permitir:

- crear un árbol con una capacidad máxima por nodo;
- buscar una clave;
- insertar una clave con un puntero lógico de registro;
- validar invariantes internas;
- reportar errores de representación sin pánico inesperado.

El objetivo no es diseñar un índice de producción. El objetivo es tener tipos
pequeños, explícitos y fáciles de probar con TDD.

## Alternativas Consideradas

### API genérica sobre `K` y `V`

Permitir `BTree<K, V>` desde el inicio haría el diseño más flexible, pero
también agregaría ruido de traits, ownership y casos de prueba que no enseñan
el mecanismo principal. Para este curso, eso distrae.

### Claves textuales desde el inicio

Las claves textuales acercan el ejemplo a casos reales, pero obligan a hablar
de collation, normalización, tamaño variable y serialización. Esos temas
pertenecen a capítulos posteriores o a la secuela de motores reales.

### Modelo educativo con clave numérica

Una clave numérica permite concentrarse en orden, fanout, búsqueda, inserción y
split. Es la opción inicial más clara.

## Decisión De Representación

La API mínima del capítulo 01 usará estos tipos públicos:

```rust
pub struct BTree;
pub struct NodeId(u64);
pub struct Key(u64);
pub struct RecordPointer {
    pub page_id: u64,
    pub slot_id: u16,
}
pub enum BTreeError;
```

La implementación concreta queda para los siguientes issues, pero estos nombres
son la frontera estable del capítulo.

## Contrato Público Propuesto

El primer contrato funcional debe crecer en este orden:

```rust
impl BTree {
    pub fn new(max_keys_per_node: usize) -> Result<Self, BTreeError>;
    pub fn is_empty(&self) -> bool;
    pub fn len(&self) -> usize;
    pub fn height(&self) -> usize;
    pub fn search(&self, key: Key) -> Result<Option<RecordPointer>, BTreeError>;
    pub fn insert(
        &mut self,
        key: Key,
        pointer: RecordPointer,
    ) -> Result<(), BTreeError>;
    pub fn root_separator(&self) -> Option<Key>;
    pub fn leaf_keys(&self) -> Vec<Vec<Key>>;
    pub fn validate(&self) -> Result<(), BTreeError>;
}
```

`max_keys_per_node` se prefiere sobre `order` porque evita ambigüedad. En la
literatura, "orden" puede significar máximo de hijos, mínimo grado o capacidad
de claves, según la fuente. El curso debe enseñar esa diferencia en el capítulo
y usar un nombre que diga exactamente qué controla.

## Tipos

### `BTree`

Representa el índice completo. En la primera implementación será un árbol en
memoria. No promete persistencia, concurrencia ni recuperación ante fallos.

Responsabilidades iniciales:

- conservar la raíz;
- registrar el número de pares clave-puntero;
- delegar búsqueda e inserción a nodos internos;
- exponer `validate` como herramienta educativa.

### `NodeId`

Identificador lógico de nodo. Aunque el primer modelo viva en memoria, `NodeId`
prepara el terreno para hablar de páginas, referencias internas y errores de
representación.

Reglas:

- no se expone como índice de vector;
- no debe depender de direcciones de memoria;
- no representa una página real en disco todavía.

### `Key`

Clave comparable del índice. Será un `u64` para mantener determinismo y evitar
temas de normalización textual en el primer capítulo.

Reglas:

- el orden total de `Key` define el orden del árbol;
- la primera versión rechazará claves duplicadas;
- las claves duplicadas se estudiarán después al hablar de índices no únicos.

### `RecordPointer`

Puntero lógico al registro indexado.

```rust
pub struct RecordPointer {
    pub page_id: u64,
    pub slot_id: u16,
}
```

No es una dirección de memoria. Es una abstracción educativa parecida a "página
y ranura", suficiente para explicar que un índice no necesariamente contiene el
registro completo.

### `BTreeError`

Errores mínimos previstos:

```rust
pub enum BTreeError {
    InvalidMaxKeysPerNode { max_keys_per_node: usize },
    DuplicateKey(Key),
    NodeFullRequiresSplit { max_keys_per_node: usize },
    MissingNode(NodeId),
    InvariantViolation(&'static str),
}
```

`InvariantViolation` existe para que `validate` pueda enseñar fallas internas
con mensajes cortos. No debe usarse para esconder errores esperados por API.

## Invariantes Iniciales

- Las claves de cada nodo están estrictamente ordenadas.
- Un nodo no raíz nunca excede `max_keys_per_node`.
- La raíz puede estar vacía solo cuando el árbol completo está vacío.
- Todas las hojas viven a la misma profundidad.
- Cada separación entre hijos respeta rangos de claves.
- `len` cuenta pares clave-puntero, no nodos.
- `height` es `0` para un árbol vacío y `1` para un árbol con solo raíz.
- La primera versión no acepta claves duplicadas.

## Inspección Educativa

El capítulo puede exponer métodos de inspección, aunque no serían parte normal
de un índice de producción:

- `root_separator` permite observar la clave separadora promovida después del
  primer split.
- `leaf_keys` permite verificar que las hojas conservan las claves ordenadas de
  izquierda a derecha.

Estos métodos existen para enseñar representación e invariantes. No sustituyen
los métodos normales de lectura como `search`.

## Límites Explícitos Del Capítulo 01

Quedan fuera de esta API inicial:

- eliminación;
- rangos;
- claves duplicadas;
- serialización de páginas;
- caché de páginas;
- concurrencia;
- WAL;
- recovery;
- compactación;
- optimización de consultas.

Estos temas aparecen después en índices, transacciones, WAL, recovery y query
optimizer. Mantenerlos fuera ayuda a que B-Tree enseñe una sola idea central:
mantener búsquedas eficientes con nodos ordenados y altura balanceada.

## Secuencia TDD Derivada

La implementación debe seguir este orden de issues:

1. Test rojo para búsqueda en árbol vacío.
2. Implementación mínima de búsqueda.
3. Test rojo para inserción simple.
4. Implementación de inserción sin split.
5. Test rojo para split de nodo.
6. Implementación educativa de split.
7. Documentación de invariantes.

Cada paso conserva la regla del repositorio: un issue, un commit y un PR.

## Relación Con Motores Reales

PostgreSQL, MySQL y SQLite usan variantes de árboles balanceados para índices,
pero el capítulo no depende de ellos. Las comparaciones deben aparecer como
contexto, no como autoridad que sustituye el modelo Rust del curso.

La distinción entre B-Tree y B+Tree debe documentarse en `docs/01-btree.md`. El
modelo inicial puede simplificar detalles de producción, siempre que declare
sus límites y mantenga sus invariantes verificables.
