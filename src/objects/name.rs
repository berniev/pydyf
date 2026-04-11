/// Spec:
/// name object:
///     an atomic symbol uniquely defined by a sequence of characters introduced by a SOLIDUS (/),
///     (2Fh) but the SOLIDUS is not considered to be part of the name
///
/// name tree:
///     similar to a dictionary that associates keys and values but the keys in a name tree are
///     strings and are ordered
///
use crate::PdfError;

#[derive(Clone)]
pub struct PdfNameObject {
    pub(crate) value: String,
    pub(crate) object_number: Option<u64>,
    pub(crate) generation_number: Option<u16>,
}

impl PdfNameObject {
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
        Ok(format!("/{}", self.value).into_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_simple_name() {
        let obj = PdfNameObject::new("Type");
        assert_eq!(obj.encode().unwrap(), b"/Type");
    }

    #[test]
    fn encode_longer_name() {
        let obj = PdfNameObject::new("FlateDecode");
        assert_eq!(obj.encode().unwrap(), b"/FlateDecode");
    }

    #[test]
    fn encode_empty_name() {
        let obj = PdfNameObject::new("");
        assert_eq!(obj.encode().unwrap(), b"/");
    }
}