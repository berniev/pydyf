//--------------------------- PdfBooleanObject----------------------//

use crate::PdfError;

#[derive(Clone)]
pub struct PdfBooleanObject {
    pub value: bool,
}

impl PdfBooleanObject {
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    pub fn set(&mut self, value: bool) {
        self.value = value;
    }

    pub fn serialise(&mut self) -> Result<Vec<u8>, PdfError> {
        let value = if self.value { "true" } else { "false" };

        Ok(Vec::from(value))
    }
}
