use crate::PdfError;
use crate::version::Version;

#[derive(Clone)]
pub struct PdfBooleanObject {
    pub(crate) value: bool,
}

impl PdfBooleanObject {
    pub fn new(value: bool) -> Self {
        Self {
            value,
        }
    }

    pub fn set(&mut self, value: bool) {
        self.value = value;
    }

    pub fn encode(&self, _version: Version) -> Result<Vec<u8>, PdfError> {
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
        assert_eq!(obj.encode(Version::V1_5).unwrap(), b"true");
    }

    #[test]
    fn encode_false() {
        let obj = PdfBooleanObject::new(false);
        assert_eq!(obj.encode(Version::V1_5).unwrap(), b"false");
    }

    #[test]
    fn encode_after_set() {
        let mut obj = PdfBooleanObject::new(true);
        obj.set(false);
        assert_eq!(obj.encode(Version::V1_5).unwrap(), b"false");
    }
}