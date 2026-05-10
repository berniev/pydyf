use crate::object_ops::{ObjectNumber, PdfEncode, write_indirect_object, serialize_object};
use crate::objects::pdf_number::PdfNumberObject;
use crate::version::Version;
use crate::xref_ops::XRefOps;
use crate::{PdfDictionaryObject, PdfError, PdfNullObject, PdfStreamObject};
use std::fs::File;

pub struct PdfArrayObject {
    pub(crate) object_number: Option<ObjectNumber>,
    pub(crate) values: Vec<Box<dyn PdfEncode>>,
}

impl PdfArrayObject {
    pub fn new() -> Self {
        Self {
            object_number: None,
            values: vec![],
        }
    }

    pub fn from_vec_number<T>(values: Vec<T>) -> PdfArrayObject
    where
        T: Into<PdfNumberObject>,
    {
        PdfArrayObject {
            object_number: None,
            values: values
                .into_iter()
                .map(|v| Box::new(v.into()) as Box<dyn PdfEncode>)
                .collect(),
        }
    }

    pub fn push_num_or_null(&mut self, v: Option<f64>) {
        match v {
            Some(n) => self.values.push(Box::new(PdfNumberObject::from(n))),
            None => self.values.push(Box::new(PdfNullObject {})),
        }
    }

    pub fn to_vec_f64(&self) -> Result<Vec<f64>, PdfError> {
        self.values
            .iter()
            .map(|v| {
                v.as_any()
                    .downcast_ref::<PdfNumberObject>()
                    .map(|n| n.as_real())
                    .ok_or_else(|| PdfError::StructureError("expected Number".to_string()))
            })
            .collect()
    }

    pub fn push(&mut self, value: impl PdfEncode + 'static) {
        self.values.push(Box::new(value));
    }

    pub fn serialize(
        &mut self,
        version: Version,
        xref: &mut XRefOps,
        file: &mut File,
    ) -> Result<(), PdfError> {
        if self.object_number.is_some() {
            write_indirect_object(
                self.object_number.unwrap(),
                self.encode(version)?,
                xref,
                file,
            )?;
        };

        for value in &mut self.values {
            serialize_object(value, version, xref, file)?;
        }

        Ok(())
    }

    pub fn encode(&mut self, version: Version) -> Result<Vec<u8>, PdfError> {
        let mut arr = vec![];
        arr.push(b'[');
        arr.push(b' ');
        for pdf_object in &mut self.values {
            arr.extend(pdf_object.pdf_encode(version)?);
            arr.push(b' ');
        }
        arr.push(b']');

        Ok(arr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PdfNameObject;
    use crate::objects::pdf_reference::PdfReferenceObject;

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
        arr.push(PdfReferenceObject::new(ObjectNumber::new(10)));
        assert_eq!(arr.encode(Version::V1_5).unwrap(), b"[ 10 0 R ]");
    }
}
