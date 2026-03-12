use crate::PdfObject;
use crate::PdfMetadata;

/// Spec:
/// name object:
///     an atomic symbol uniquely defined by a sequence of characters introduced by a SOLIDUS (/),
///     (2Fh) but the SOLIDUS is not considered to be part of the name
///
/// name tree:
///     similar to a dictionary that associates keys and values but the keys in a name tree are
///     strings and are ordered

pub struct NameObject {
    metadata: PdfMetadata,
    pub value: String,
}

impl NameObject {
    pub fn new(value: String) -> Self {
        Self {
            metadata: Default::default(),
            value,
        }
    }
}

impl PdfObject for NameObject {
    fn data(&self) -> String {
        format!("/{}", self.value)
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }
}
