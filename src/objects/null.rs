use crate::PdfError;

#[derive(Clone)]
pub struct PdfNullObject {}

impl PdfNullObject {
    pub fn new() -> Self {
        Self {}
    }
}

impl PdfNullObject {
    pub fn serialise(&mut self) -> Result<Vec<u8>, PdfError> {
        Ok(vec![])
    }
}
