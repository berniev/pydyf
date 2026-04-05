use crate::PdfError;

#[derive(Clone)]
pub struct PdfNullObject {
    pub(crate) object_number: Option<u64>,
}

impl PdfNullObject {
    pub fn new() -> Self {
        Self {
            object_number: None,
        }
    }
}

impl PdfNullObject {
    pub fn serialise(&self) -> Result<Vec<u8>, PdfError> {
        Ok(vec![])
    }
}
