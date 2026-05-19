use crate::object_ops::{Encode, PdfObject, Serialize};
use crate::PdfError;
use crate::version::Version;

#[derive(Clone)]
pub struct PdfBooleanObject {
    pub(crate) value: bool,
}

impl PdfBooleanObject {
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    pub fn set(&mut self, value: bool) {
        self.value = value;
    }
}

impl Encode for PdfBooleanObject {
    fn encode(&self, _version: Version) -> Result<Vec<u8>, PdfError> {
        Ok(if self.value {
            Vec::from(b"true")
        } else {
            Vec::from(b"false")
        })
    }
}

impl Serialize for PdfBooleanObject {}

impl From<bool> for Box<dyn PdfObject> {
    fn from(v: bool) -> Self {
        Box::new(PdfBooleanObject::new(v))
    }
}
