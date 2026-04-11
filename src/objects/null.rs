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
    
    pub fn with_object_number(mut self, value: u64) -> Self {
        self.object_number = Some(value);
        self
    }

    pub fn with_generation_number(mut self, value: u16) -> Self {
        self.generation_number = Some(value); 
        self
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