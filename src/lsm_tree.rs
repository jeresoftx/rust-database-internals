//! Modelo educativo inicial de un LSM Tree.
//!
//! Este módulo arranca con la frontera mínima del capítulo: nombrar las piezas
//! que sostienen el mecanismo. Una LSM Tree escribe primero en memoria
//! (`MemTable`), congela esa memoria en segmentos ordenados e inmutables
//! (`SSTable`) y compacta varios segmentos en uno nuevo (`CompactionPlan`).
//! Las operaciones de escritura, búsqueda, flush y compaction quedan para
//! pasos posteriores del capítulo.

use std::collections::HashSet;

/// Tabla mutable en memoria donde aterrizan las escrituras recientes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemTable {
    max_entries: usize,
    len: usize,
}

impl MemTable {
    /// Crea una MemTable vacía con una capacidad máxima de entradas.
    pub fn new(max_entries: usize) -> Result<Self, LsmTreeError> {
        if max_entries == 0 {
            return Err(LsmTreeError::InvalidMemTableCapacity { max_entries });
        }

        Ok(Self {
            max_entries,
            len: 0,
        })
    }

    /// Devuelve `true` cuando la MemTable no contiene entradas.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Devuelve cuántas entradas contiene la MemTable.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Capacidad máxima de entradas antes de requerir flush.
    pub fn max_entries(&self) -> usize {
        self.max_entries
    }

    /// Devuelve `true` cuando todavía cabe una entrada más.
    pub fn can_accept_entry(&self) -> bool {
        self.len < self.max_entries
    }
}

/// Segmento ordenado e inmutable producido por un flush o una compaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SSTable {
    segment_id: SegmentId,
    key_count: usize,
}

impl SSTable {
    /// Crea metadatos de una SSTable educativa.
    pub fn new(segment_id: SegmentId, key_count: usize) -> Self {
        Self {
            segment_id,
            key_count,
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
    /// deben repetirse. El algoritmo de merge se modelará después.
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
