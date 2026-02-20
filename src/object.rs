use std::fmt;

/// PDF object status as specified in the cross-reference table.
/// Maps directly to the single-character status in the PDF xref table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectStatus {
    /// Free object slot (deleted or never used) - outputs 'f' in xref table
    Free,
    /// In-use object (normal, active object) - outputs 'n' in xref table
    InUse,
}

impl ObjectStatus {
    /// Returns the PDF character representation ('f' or 'n')
    pub fn as_char(&self) -> char {
        match self {
            ObjectStatus::Free => 'f',
            ObjectStatus::InUse => 'n',
        }
    }
}

impl fmt::Display for ObjectStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

#[derive(Debug, Clone)]
pub struct PdfMetadata {
    /// Object number in the PDF (None for unassigned objects)
    pub number: Option<usize>,

    /// Byte offset in the PDF file (used in cross-reference table)
    pub offset: usize,

    /// PDF generation number. Almost always 0 for new objects.
    /// - 0 = original/current version (standard for all new objects)
    /// - 65535 = special value for the free object 0 (PDF spec requirement)
    /// - 1+ = incremental updates (rarely used in modern PDFs)
    pub generation: u32,

    /// Object status: Free (deleted/unused) or InUse (normal)
    pub status: ObjectStatus,
}

impl Default for PdfMetadata {
    fn default() -> Self {
        PdfMetadata {
            number: None,
            offset: 0,
            generation: 0,
            status: ObjectStatus::InUse,
        }
    }
}

impl PdfMetadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_free() -> Self {
        PdfMetadata {
            status: ObjectStatus::Free,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    pub metadata: PdfMetadata,
}

impl Object {
    pub fn new() -> Self {
        Object {
            metadata: PdfMetadata::new_free(),
        }
    }
}

impl PdfObject for Object {
    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PdfMetadata {
        &mut self.metadata
    }

    fn data(&self) -> Vec<u8> {
        // Base Object has no data - used for free/placeholder objects
        Vec::new()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn compressible(&self) -> bool {
        self.metadata.generation == 0
    }
}

/// Common trait for PDF objects
pub trait PdfObject {
    /// Get immutable reference to metadata
    fn metadata(&self) -> &PdfMetadata;

    /// Get mutable reference to metadata
    fn metadata_mut(&mut self) -> &mut PdfMetadata;

    /// Get the object's data as bytes
    fn data(&self) -> Vec<u8>;

    /// Downcast to Any for type checking
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    /// Indirect representation of an object.
    fn indirect(&self) -> Vec<u8> {
        let meta = self.metadata();
        let number = meta.number.unwrap_or(0);
        let header = format!("{} {} obj\n", number, meta.generation);
        let mut result = header.into_bytes();
        result.extend(self.data());
        result.extend(b"\nendobj");
        result
    }

    /// Object reference.
    fn reference(&self) -> Vec<u8> {
        let meta = self.metadata();
        let number = meta.number.unwrap_or(0);
        format!("{} {} R", number, meta.generation).into_bytes()
    }

    /// Whether the object can be included in an object stream (PDF 1.5+).
    ///
    /// PDF spec: Only generation 0 objects can be compressed in object streams.
    /// Objects with generation > 0 (incremental updates) must be written directly.
    ///
    /// Note: Some object types (like Stream) override this to always return false.
    fn compressible(&self) -> bool {
        self.metadata().generation == 0
    }
}
