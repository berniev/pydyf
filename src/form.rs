use crate::{PdfDictionaryObject, PdfError};

pub struct Form {
    pub dict:PdfDictionaryObject
}

impl Form {
    pub fn new() -> Result<Self, PdfError> {
        Ok(Self { dict: PdfDictionaryObject::new() })
    }

    pub fn with_type(mut self, type_str: &str) -> Result<Self, PdfError> {
        self.dict.add("Type", type_str)?;
        Ok(self)
    }

    // todo: lots of others
}