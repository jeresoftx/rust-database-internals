//! Modelo educativo inicial de un B-Tree.
//!
//! Este módulo comienza con la frontera mínima del capítulo: crear un árbol
//! vacío, exponer sus métricas básicas y buscar una clave. Los nodos reales,
//! inserciones y splits se agregan en los siguientes pasos del plan.

/// Índice balanceado por altura para buscar punteros de registro por clave.
#[derive(Debug, Clone)]
pub struct BTree {
    max_keys_per_node: usize,
    entries: Vec<(Key, RecordPointer)>,
}

impl BTree {
    /// Crea un B-Tree vacío.
    ///
    /// `max_keys_per_node` define cuántas claves puede contener un nodo antes
    /// de necesitar un split. El mínimo educativo inicial es `3`: con valores
    /// menores no se puede mostrar un split útil en el capítulo.
    pub fn new(max_keys_per_node: usize) -> Result<Self, BTreeError> {
        if max_keys_per_node < 3 {
            return Err(BTreeError::InvalidMaxKeysPerNode { max_keys_per_node });
        }

        Ok(Self {
            max_keys_per_node,
            entries: Vec::new(),
        })
    }

    /// Devuelve `true` cuando el árbol no contiene pares clave-puntero.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Devuelve el número de pares clave-puntero almacenados.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Devuelve la altura del árbol.
    ///
    /// Un árbol vacío tiene altura `0`. Cuando exista una raíz, la altura será
    /// al menos `1`.
    pub fn height(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            1
        }
    }

    /// Busca una clave y devuelve el puntero asociado cuando existe.
    pub fn search(&self, key: Key) -> Result<Option<RecordPointer>, BTreeError> {
        match self.find_key(key) {
            Ok(position) => Ok(Some(self.entries[position].1)),
            Err(_) => Ok(None),
        }
    }

    /// Inserta una clave con su puntero de registro.
    ///
    /// Esta primera versión solo cubre una raíz-hoja con capacidad disponible.
    /// Cuando el nodo está lleno, el split se implementa en un paso posterior.
    pub fn insert(&mut self, key: Key, pointer: RecordPointer) -> Result<(), BTreeError> {
        match self.find_key(key) {
            Ok(_) => Err(BTreeError::DuplicateKey(key)),
            Err(position) => {
                if self.entries.len() >= self.max_keys_per_node {
                    return Err(BTreeError::NodeFullRequiresSplit {
                        max_keys_per_node: self.max_keys_per_node,
                    });
                }

                self.entries.insert(position, (key, pointer));
                Ok(())
            }
        }
    }

    /// Capacidad máxima de claves por nodo.
    pub fn max_keys_per_node(&self) -> usize {
        self.max_keys_per_node
    }

    fn find_key(&self, key: Key) -> Result<usize, usize> {
        self.entries
            .binary_search_by_key(&key, |(entry_key, _)| *entry_key)
    }
}

/// Identificador lógico de nodo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(u64);

impl NodeId {
    /// Crea un identificador lógico de nodo.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico del identificador.
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// Clave comparable del índice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key(u64);

impl Key {
    /// Crea una clave numérica.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico de la clave.
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// Puntero lógico al registro indexado.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RecordPointer {
    /// Página lógica donde vive el registro.
    pub page_id: u64,
    /// Ranura lógica dentro de la página.
    pub slot_id: u16,
}

/// Errores del modelo educativo de B-Tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BTreeError {
    /// La capacidad máxima por nodo no permite enseñar splits útiles.
    InvalidMaxKeysPerNode { max_keys_per_node: usize },
    /// La primera versión del capítulo no acepta claves duplicadas.
    DuplicateKey(Key),
    /// El nodo raíz está lleno y el siguiente paso debe aplicar split.
    NodeFullRequiresSplit { max_keys_per_node: usize },
    /// La representación interna apunta a un nodo que no existe.
    MissingNode(NodeId),
    /// Una invariante interna fue violada.
    InvariantViolation(&'static str),
}
