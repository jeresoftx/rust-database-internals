//! Modelo educativo inicial de un LSM Tree.
//!
//! Este módulo arranca con la frontera mínima del capítulo: nombrar las piezas
//! que sostienen el mecanismo. Una LSM Tree escribe primero en memoria
//! (`MemTable`), congela esa memoria en segmentos ordenados e inmutables
//! (`SSTable`) y compacta varios segmentos en uno nuevo (`CompactionPlan`).
//! El modelo actual ya acepta escrituras en memoria, flush hacia SSTable,
//! búsqueda por precedencia y compaction educativa.

use std::collections::{BTreeMap, HashSet};

/// Árbol LSM educativo con MemTable activa y segmentos inmutables.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LsmTree {
    memtable: MemTable,
    segments: Vec<SSTable>,
}

impl LsmTree {
    /// Crea un LSM Tree vacío.
    pub fn new(memtable_max_entries: usize) -> Result<Self, LsmTreeError> {
        Ok(Self {
            memtable: MemTable::new(memtable_max_entries)?,
            segments: Vec::new(),
        })
    }

    /// Escribe o reemplaza una clave en la MemTable activa.
    pub fn write(&mut self, key: LsmKey, value: LsmValue) -> Result<(), LsmTreeError> {
        self.memtable.write(key, value)
    }

    /// Congela la MemTable activa y agrega el segmento al final.
    pub fn flush_to_sstable(&mut self, segment_id: SegmentId) -> Result<(), LsmTreeError> {
        let sstable = self.memtable.flush_to_sstable(segment_id)?;
        self.segments.push(sstable);
        Ok(())
    }

    /// Busca primero en MemTable y luego en SSTables, de más reciente a más
    /// antigua.
    pub fn search(&self, key: LsmKey) -> Option<LsmValue> {
        self.memtable.search(key).or_else(|| {
            self.segments
                .iter()
                .rev()
                .find_map(|segment| segment.search(key))
        })
    }

    /// Compacta segmentos existentes en un nuevo segmento inmutable.
    ///
    /// La compaction conserva la versión visible más reciente de cada clave:
    /// si una clave aparece en varios segmentos de entrada, gana el segmento
    /// que fue creado después dentro del árbol. Los segmentos fuera del plan
    /// se conservan en su orden original y el resultado compactado se agrega
    /// como el segmento más reciente.
    pub fn compact(&mut self, plan: CompactionPlan) -> Result<(), LsmTreeError> {
        for input_segment in plan.input_segments() {
            if !self
                .segments
                .iter()
                .any(|segment| segment.segment_id() == *input_segment)
            {
                return Err(LsmTreeError::MissingSegment(*input_segment));
            }
        }

        if self
            .segments
            .iter()
            .any(|segment| segment.segment_id() == plan.output_segment())
        {
            return Err(LsmTreeError::OutputSegmentConflicts(plan.output_segment()));
        }

        let input_segments: HashSet<_> = plan.input_segments().iter().copied().collect();
        let mut compacted_entries = BTreeMap::new();

        for segment in &self.segments {
            if input_segments.contains(&segment.segment_id()) {
                for (key, value) in segment.entries() {
                    compacted_entries.insert(key, value);
                }
            }
        }

        let output_segment = SSTable::from_entries(
            plan.output_segment(),
            compacted_entries.into_iter().collect(),
        );

        self.segments
            .retain(|segment| !input_segments.contains(&segment.segment_id()));
        self.segments.push(output_segment);

        Ok(())
    }

    /// Segmentos inmutables en orden de creación.
    pub fn segments(&self) -> &[SSTable] {
        &self.segments
    }
}

/// Tabla mutable en memoria donde aterrizan las escrituras recientes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemTable {
    max_entries: usize,
    entries: BTreeMap<LsmKey, LsmValue>,
}

impl MemTable {
    /// Crea una MemTable vacía con una capacidad máxima de entradas.
    pub fn new(max_entries: usize) -> Result<Self, LsmTreeError> {
        if max_entries == 0 {
            return Err(LsmTreeError::InvalidMemTableCapacity { max_entries });
        }

        Ok(Self {
            max_entries,
            entries: BTreeMap::new(),
        })
    }

    /// Devuelve `true` cuando la MemTable no contiene entradas.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Devuelve cuántas entradas contiene la MemTable.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Capacidad máxima de entradas antes de requerir flush.
    pub fn max_entries(&self) -> usize {
        self.max_entries
    }

    /// Devuelve `true` cuando todavía cabe una entrada más.
    pub fn can_accept_entry(&self) -> bool {
        self.len() < self.max_entries
    }

    /// Escribe o reemplaza una clave dentro de la MemTable.
    ///
    /// Reemplazar una clave existente no consume capacidad adicional. Escribir
    /// una clave nueva cuando la MemTable está llena devuelve un error para
    /// que el siguiente paso del capítulo pueda modelar flush explícitamente.
    pub fn write(&mut self, key: LsmKey, value: LsmValue) -> Result<(), LsmTreeError> {
        if !self.entries.contains_key(&key) && !self.can_accept_entry() {
            return Err(LsmTreeError::MemTableFull {
                max_entries: self.max_entries,
            });
        }

        self.entries.insert(key, value);
        Ok(())
    }

    /// Devuelve las entradas actuales en orden ascendente por clave.
    pub fn entries(&self) -> Vec<(LsmKey, LsmValue)> {
        self.entries
            .iter()
            .map(|(key, value)| (*key, value.clone()))
            .collect()
    }

    /// Busca una clave dentro de la MemTable.
    pub fn search(&self, key: LsmKey) -> Option<LsmValue> {
        self.entries.get(&key).cloned()
    }

    /// Congela las entradas actuales en una SSTable inmutable y vacía memoria.
    pub fn flush_to_sstable(&mut self, segment_id: SegmentId) -> Result<SSTable, LsmTreeError> {
        if self.is_empty() {
            return Err(LsmTreeError::EmptyMemTableFlush);
        }

        let entries = self.entries();
        self.entries.clear();

        Ok(SSTable::from_entries(segment_id, entries))
    }
}

/// Clave comparable de una MemTable o SSTable educativa.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LsmKey(u64);

impl LsmKey {
    /// Crea una clave numérica para el modelo LSM.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico de la clave.
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// Valor almacenado por una escritura en memoria.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LsmValue(Vec<u8>);

impl LsmValue {
    /// Crea un valor desde bytes.
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    /// Devuelve los bytes del valor.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl From<&str> for LsmValue {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().to_vec())
    }
}

/// Segmento ordenado e inmutable producido por un flush o una compaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SSTable {
    segment_id: SegmentId,
    key_count: usize,
    entries: Vec<(LsmKey, LsmValue)>,
}

impl SSTable {
    /// Crea metadatos de una SSTable educativa.
    pub fn new(segment_id: SegmentId, key_count: usize) -> Self {
        Self {
            segment_id,
            key_count,
            entries: Vec::new(),
        }
    }

    /// Crea una SSTable desde entradas ya ordenadas.
    pub fn from_entries(segment_id: SegmentId, entries: Vec<(LsmKey, LsmValue)>) -> Self {
        let key_count = entries.len();

        Self {
            segment_id,
            key_count,
            entries,
        }
    }

    /// Identificador lógico del segmento.
    pub fn segment_id(&self) -> SegmentId {
        self.segment_id
    }

    /// Número de claves visibles en el segmento.
    pub fn key_count(&self) -> usize {
        self.key_count
    }

    /// Devuelve `true` cuando el segmento no contiene claves.
    pub fn is_empty(&self) -> bool {
        self.key_count == 0
    }

    /// Devuelve el snapshot ordenado almacenado en el segmento.
    pub fn entries(&self) -> Vec<(LsmKey, LsmValue)> {
        self.entries.clone()
    }

    /// Busca una clave dentro del snapshot del segmento.
    pub fn search(&self, key: LsmKey) -> Option<LsmValue> {
        self.entries
            .binary_search_by_key(&key, |(entry_key, _)| *entry_key)
            .ok()
            .map(|position| self.entries[position].1.clone())
    }
}

/// Identificador lógico de segmento SSTable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SegmentId(u64);

impl SegmentId {
    /// Crea un identificador lógico de segmento.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico del identificador.
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// Plan educativo para compactar segmentos de entrada en un segmento nuevo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactionPlan {
    input_segments: Vec<SegmentId>,
    output_segment: SegmentId,
}

impl CompactionPlan {
    /// Crea un plan de compaction.
    ///
    /// La salida debe ser un segmento nuevo y los segmentos de entrada no
    /// deben repetirse. La existencia de esos segmentos se valida contra el
    /// árbol al ejecutar la compaction.
    pub fn new(
        input_segments: Vec<SegmentId>,
        output_segment: SegmentId,
    ) -> Result<Self, LsmTreeError> {
        if input_segments.is_empty() {
            return Err(LsmTreeError::EmptyCompactionInput);
        }

        let mut seen = HashSet::with_capacity(input_segments.len());
        for segment in &input_segments {
            if !seen.insert(*segment) {
                return Err(LsmTreeError::DuplicateCompactionInput(*segment));
            }
        }

        if seen.contains(&output_segment) {
            return Err(LsmTreeError::OutputSegmentConflicts(output_segment));
        }

        Ok(Self {
            input_segments,
            output_segment,
        })
    }

    /// Segmentos que se leerán durante la compaction.
    pub fn input_segments(&self) -> &[SegmentId] {
        &self.input_segments
    }

    /// Segmento nuevo que recibirá el resultado de la compaction.
    pub fn output_segment(&self) -> SegmentId {
        self.output_segment
    }
}

/// Errores del modelo educativo de LSM Tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LsmTreeError {
    /// La MemTable necesita capacidad positiva para enseñar flush.
    InvalidMemTableCapacity { max_entries: usize },
    /// Una compaction sin entradas no produciría aprendizaje ni datos.
    EmptyCompactionInput,
    /// Un flush vacío produciría un segmento sin frontera educativa clara.
    EmptyMemTableFlush,
    /// Un segmento aparece más de una vez como entrada.
    DuplicateCompactionInput(SegmentId),
    /// El segmento de salida ya aparece como entrada.
    OutputSegmentConflicts(SegmentId),
    /// La MemTable está llena y el siguiente paso debe aplicar flush.
    MemTableFull { max_entries: usize },
    /// La búsqueda apunta a un segmento que no existe.
    MissingSegment(SegmentId),
    /// Una invariante interna fue violada.
    InvariantViolation(&'static str),
}
