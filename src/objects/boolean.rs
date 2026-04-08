//--------------------------- PdfBooleanObject----------------------//

use crate::PdfError;

#[derive(Clone)]
pub struct PdfBooleanObject {
    pub(crate) value: bool,
    pub(crate) object_number: Option<u64>,
    pub(crate) generation_number: Option<u16>,
}

impl PdfBooleanObject {
    pub fn new(value: bool) -> Self {
        Self {
            value,
            object_number: None,
            generation_number: None,
        }
    }

    pub fn set(&mut self, value: bool) {
        self.value = value;
    }

    pub fn encode(&self) -> Result<Vec<u8>, PdfError> {
        let value = if self.value { "true" } else { "false" };

        Ok(Vec::from(value))
    }
}
