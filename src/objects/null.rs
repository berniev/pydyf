use crate::PdfError;

#[derive(Clone)]
pub struct PdfNullObject {
    pub(crate) object_number: Option<u64>,
    pub(crate) generation_number: Option<u16>,
}

impl PdfNullObject {
    pub fn new() -> Self {
        Self {
            object_number: None,
            generation_number: None,
        }
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