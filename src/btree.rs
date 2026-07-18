//! Modelo educativo inicial de un B-Tree.
//!
//! Este módulo comienza con la frontera mínima del capítulo: crear un árbol
//! vacío, exponer sus métricas básicas, buscar una clave e ilustrar el primer
//! split de raíz. Los splits recursivos quedan para capítulos posteriores.

/// Índice balanceado por altura para buscar punteros de registro por clave.
#[derive(Debug, Clone)]
pub struct BTree {
    max_keys_per_node: usize,
    root: Root,
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
            root: Root::Leaf(Vec::new()),
        })
    }

    /// Devuelve `true` cuando el árbol no contiene pares clave-puntero.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Devuelve el número de pares clave-puntero almacenados.
    pub fn len(&self) -> usize {
        match &self.root {
            Root::Leaf(entries) => entries.len(),
            Root::Internal { left, right, .. } => left.len() + right.len(),
        }
    }

    /// Devuelve la altura del árbol.
    ///
    /// Un árbol vacío tiene altura `0`. Cuando exista una raíz, la altura será
    /// al menos `1`.
    pub fn height(&self) -> usize {
        match &self.root {
            Root::Leaf(entries) if entries.is_empty() => 0,
            Root::Leaf(_) => 1,
            Root::Internal { .. } => 2,
        }
    }

    /// Busca una clave y devuelve el puntero asociado cuando existe.
    pub fn search(&self, key: Key) -> Result<Option<RecordPointer>, BTreeError> {
        let entries = self.entries_for_search(key);

        match find_key(entries, key) {
            Ok(position) => Ok(Some(entries[position].pointer)),
            Err(_) => Ok(None),
        }
    }

    /// Inserta una clave con su puntero de registro.
    ///
    /// Esta versión cubre una raíz-hoja con capacidad disponible y el primer
    /// split de raíz. Si una hoja posterior se llena, el split recursivo queda
    /// como trabajo futuro explícito.
    pub fn insert(&mut self, key: Key, pointer: RecordPointer) -> Result<(), BTreeError> {
        match &mut self.root {
            Root::Leaf(entries) => {
                insert_into_root_leaf(entries, self.max_keys_per_node, key, pointer).map(|split| {
                    if let Some((left, separator, right)) = split {
                        self.root = Root::Internal {
                            separator,
                            left,
                            right,
                        };
                    }
                })
            }
            Root::Internal {
                separator,
                left,
                right,
            } => {
                let target = if key < *separator { left } else { right };
                match find_key(target, key) {
                    Ok(_) => Err(BTreeError::DuplicateKey(key)),
                    Err(position) => {
                        if target.len() >= self.max_keys_per_node {
                            return Err(BTreeError::NodeFullRequiresSplit {
                                max_keys_per_node: self.max_keys_per_node,
                            });
                        }

                        target.insert(position, Entry { key, pointer });
                        Ok(())
                    }
                }
            }
        }
    }

    /// Capacidad máxima de claves por nodo.
    pub fn max_keys_per_node(&self) -> usize {
        self.max_keys_per_node
    }

    /// Devuelve la clave separadora de la raíz cuando ya hubo split.
    pub fn root_separator(&self) -> Option<Key> {
        match &self.root {
            Root::Leaf(_) => None,
            Root::Internal { separator, .. } => Some(*separator),
        }
    }

    /// Devuelve las claves de cada hoja de izquierda a derecha.
    pub fn leaf_keys(&self) -> Vec<Vec<Key>> {
        match &self.root {
            Root::Leaf(entries) => vec![entries.iter().map(|entry| entry.key).collect()],
            Root::Internal { left, right, .. } => vec![
                left.iter().map(|entry| entry.key).collect(),
                right.iter().map(|entry| entry.key).collect(),
            ],
        }
    }

    fn entries_for_search(&self, key: Key) -> &[Entry] {
        match &self.root {
            Root::Leaf(entries) => entries,
            Root::Internal {
                separator,
                left,
                right,
            } => {
                if key < *separator {
                    left
                } else {
                    right
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Root {
    Leaf(Vec<Entry>),
    Internal {
        separator: Key,
        left: Vec<Entry>,
        right: Vec<Entry>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Entry {
    key: Key,
    pointer: RecordPointer,
}

type RootSplit = Option<(Vec<Entry>, Key, Vec<Entry>)>;

fn insert_into_root_leaf(
    entries: &mut Vec<Entry>,
    max_keys_per_node: usize,
    key: Key,
    pointer: RecordPointer,
) -> Result<RootSplit, BTreeError> {
    match find_key(entries, key) {
        Ok(_) => Err(BTreeError::DuplicateKey(key)),
        Err(position) => {
            if entries.len() < max_keys_per_node {
                entries.insert(position, Entry { key, pointer });
                return Ok(None);
            }

            let mut split_entries = entries.clone();
            split_entries.insert(position, Entry { key, pointer });
            Ok(Some(split_root_entries(split_entries)))
        }
    }
}

fn split_root_entries(mut entries: Vec<Entry>) -> (Vec<Entry>, Key, Vec<Entry>) {
    let split_at = entries.len() / 2;
    let right = entries.split_off(split_at);
    let separator = right[0].key;

    (entries, separator, right)
}

fn find_key(entries: &[Entry], key: Key) -> Result<usize, usize> {
    entries.binary_search_by_key(&key, |entry| entry.key)
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
