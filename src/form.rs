use crate::{PdfDictionaryObject, PdfError, PdfStringObject};

pub struct Form {
    pub dict:PdfDictionaryObject
}

impl Form {
    pub fn new() -> Result<Self, PdfError> {
        Ok(Self { dict: PdfDictionaryObject::new() })
    }

    pub fn with_type(mut self, type_str: &str) -> Result<Self, PdfError> {
        self.dict.add("Type", PdfStringObject::new(type_str))?;
        Ok(self)
    }

    // todo: lots of others
}