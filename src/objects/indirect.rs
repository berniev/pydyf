use crate::PdfObject;

struct IndirectObject {
    value: usize,
    generation_number: u16,
}

impl IndirectObject {
    pub fn new(obj: usize) -> Self {
        IndirectObject { value: obj, generation_number: 0 }
    }
}

impl PdfObject for IndirectObject {
    fn data(&self) -> String {
        format!("{} {} R", self.value, self.generation_number);
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
}
