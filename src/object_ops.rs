use crate::PdfStreamObject;

pub struct ObjectOps {
    last_object_number: u64,
}

impl ObjectOps {
    pub fn new() -> Self {
        Self {
            last_object_number: 0, // 0 is in xref table as 'free'. is gen# 65535, else 0 for new
        }
    }

    pub fn last_object_number(&self) -> u64 {
        self.last_object_number
    }

    pub fn next_object_number(&mut self) -> u64 {
        self.last_object_number += 1;

        self.last_object_number
    }

    pub fn new_stream(&mut self) -> PdfStreamObject {
        let num = self.next_object_number();
        PdfStreamObject::new().with_object_number(num)
    }
}
