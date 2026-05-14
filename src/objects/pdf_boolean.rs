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

