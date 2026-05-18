use std::fs::File;
use std::io::Write;
use crate::object_ops::{serialize_pdf_object, try_indirect_end, try_indirect_start, Encode, ObjectNumber, PdfObject, Serialize};
use crate::objects::pdf_number::PdfNumberObject;
use crate::{PdfError, PdfNullObject};
use crate::version::Version;
use crate::xref_ops::XRefOps;

pub struct PdfArrayObject {
    pub(crate) object_number: Option<ObjectNumber>,
    pub(crate) elements: Vec<Box<dyn PdfObject>>,
}

impl PdfArrayObject {
    pub fn new() -> Self {
        Self {
            object_number: None,
            elements: vec![],
        }
    }

    pub fn from_vec_number<T>(values: Vec<T>) -> PdfArrayObject
    where
        T: Into<PdfNumberObject>,
    {
        PdfArrayObject {
            object_number: None,
            elements: values
                .into_iter()
                .map(|v| Box::new(v.into()) as Box<dyn PdfObject>)
                .collect(),
        }
    }

    pub fn push_num_or_null(&mut self, v: Option<f64>) {
        match v {
            Some(n) => self.elements.push(Box::new(PdfNumberObject::from(n))),
            None => self.elements.push(Box::new(PdfNullObject {})),
        }
    }

   pub fn to_vec_i64(&self) -> Result<Vec<i64>, PdfError> {
        self.elements
            .iter()
            .map(|v| {
                v.as_any()
                    .downcast_ref::<PdfNumberObject>()
                    .map(|n| n.as_int())
                    .ok_or_else(|| PdfError::StructureError("expected Number".to_string()))
            })
            .collect()
    }

    pub fn push(&mut self, value: impl Into<Box<dyn PdfObject>>) {
        self.elements.push(value.into());
    }
}

impl Encode for PdfArrayObject {}

impl Serialize for PdfArrayObject {
    fn serialize(
        &mut self,
        version: Version,
        xref: &mut XRefOps,
        file: &mut File,
    ) -> Result<(), PdfError> {
        try_indirect_start(xref, file, self.object_number)?;

        file.write(b"[ ")?;

        for pdf_object in &mut self.elements {
            serialize_pdf_object(pdf_object, version, xref, file)?;
            file.write(b" ")?;
        }
        file.write(b"]")?;

        try_indirect_end(file, self.object_number)?;

        Ok(())
    }
}

/*
#[cfg(test)]
mod tests {

  #[test]
   fn encode_empty_array() {
       let mut arr = PdfArrayObject::new();
       assert_eq!(arr.encode(Version::V1_5).unwrap(), b"[ ]");
   }

   #[test]
   fn encode_single_element() {
       let mut arr = PdfArrayObject::new();
       arr.push(42);
       assert_eq!(arr.encode(Version::V1_5).unwrap(), b"[ 42 ]");
   }

   #[test]
   fn encode_mixed_elements() {
       let mut arr = PdfArrayObject::new();
       arr.push(549);
       arr.push(3.14);
       arr.push(false);
       assert_eq!(arr.encode(Version::V1_5).unwrap(), b"[ 549 3.14 false ]");
   }

   #[test]
   fn encode_with_name() {
       let mut arr = PdfArrayObject::new();
       arr.push(PdfNameObject::new("SomeName"));
       assert_eq!(arr.encode(Version::V1_5).unwrap(), b"[ /SomeName ]");
   }

   #[test]
   fn encode_with_indirect_reference() {
       let mut arr = PdfArrayObject::new();
       arr.push(ObjectNumber::new(10));
       assert_eq!(arr.encode(Version::V1_5).unwrap(), b"[ 10 0 R ]");
   }
}
*/
