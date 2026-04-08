//--------------------------- PdfStringObject----------------------//

use crate::PdfError;

#[derive(Clone)]
pub struct PdfStringObject {
    pub(crate) value: String,
    pub(crate) object_number: Option<u64>,
    pub(crate) generation_number: Option<u16>,
}

impl PdfStringObject {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
            object_number: None,
            generation_number: None,
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, PdfError> {
        Ok(self.value.as_bytes().to_vec())
    }
}
