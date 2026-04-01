use crate::{PdfError, PdfObject};
use std::any::Any;

pub struct PdfNullObject {}

impl PdfNullObject {
    pub fn new() -> Self {
        Self {}
    }
}

impl PdfObject for PdfNullObject {
    fn serialise(&mut self) -> Result<Vec<u8>, PdfError> {
        Ok(vec![])
    }

    fn is_indirect_by_default(&self) -> bool {
        false
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
