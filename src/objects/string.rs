use crate::PdfError;

#[derive(Clone)]
pub struct PdfStringObject {
    pub(crate) value: String,
}

impl PdfStringObject {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
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