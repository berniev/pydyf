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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_true() {
        let obj = PdfBooleanObject::new(true);
        assert_eq!(obj.encode().unwrap(), b"true");
    }

    #[test]
    fn encode_false() {
        let obj = PdfBooleanObject::new(false);
        assert_eq!(obj.encode().unwrap(), b"false");
    }

    #[test]
    fn encode_after_set() {
        let mut obj = PdfBooleanObject::new(true);
        obj.set(false);
        assert_eq!(obj.encode().unwrap(), b"false");
    }
}