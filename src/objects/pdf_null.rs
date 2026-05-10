use crate::version::Version;
use crate::PdfError;

#[derive(Clone)]
pub struct PdfNullObject {}

impl PdfNullObject {
    pub fn new() -> Self {
        Self {}
    }
}

impl PdfNullObject {
    pub fn encode(&self, _version: Version) -> Result<Vec<u8>, PdfError> {
        Ok("null".to_string().into_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_null() {
        let obj = PdfNullObject::new();
        assert_eq!(obj.encode(Version::V1_5).unwrap(), b"null");
    }
}
