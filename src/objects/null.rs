use crate::PdfError;

#[derive(Clone)]
pub struct PdfNullObject {}

impl PdfNullObject {
    pub fn new() -> Self {
        Self {}
    }
}

impl PdfNullObject {
    pub fn encode(&self) -> Result<Vec<u8>, PdfError> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_null() {
        let obj = PdfNullObject::new();
        assert_eq!(obj.encode().unwrap(), b"");
    }
}
