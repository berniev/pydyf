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

    pub fn with_object_number(mut self, value: u64) -> Self {
        self.object_number = Some(value);
        self
    }

    pub fn with_generation_number(mut self, value: u16) -> Self {
        self.generation_number = Some(value);
        self
    }

    pub fn encode(&self) -> Result<Vec<u8>, PdfError> {
        Ok(self.value.as_bytes().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_simple_string() {
        let obj = PdfStringObject::new("Hello, World!");
        assert_eq!(obj.encode().unwrap(), b"Hello, World!");
    }

    #[test]
    fn encode_empty_string() {
        let obj = PdfStringObject::new("");
        assert_eq!(obj.encode().unwrap(), b"");
    }

    #[test]
    fn encode_string_with_newline() {
        let obj = PdfStringObject::new("line1\nline2");
        assert_eq!(obj.encode().unwrap(), b"line1\nline2");
    }
}