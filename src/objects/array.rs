//! Array Objects:

///
/// An array object is a one-dimensional collection of pdf objects arranged sequentially.
///
/// Unlike arrays in many other computer languages, PDF arrays may be heterogeneous; that is, an
/// array’s elements may be any combination of numbers, strings, dictionaries, or any other pdf
/// objects, including other arrays. An array may have zero elements.
///
/// An array shall be written as a sequence of objects enclosed in SQUARE BRACKETS.
/// EXAMPLE [ 549 3.14 false ( Ralph ) /SomeName ]
///
use crate::{PdfError, PdfObject};
use crate::object_ops::ObjectNumber;

#[derive(Clone)]
pub struct PdfArrayObject {
    pub(crate) values: Vec<PdfObject>,
    pub(crate) object_number: Option<ObjectNumber>,
    pub(crate) generation_number: Option<u16>,
}

impl PdfArrayObject {
    pub fn new() -> Self {
        Self {
            values: vec![],
            object_number: None,
            generation_number: None,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn with_object_number(mut self, value: ObjectNumber) -> Self {
        self.object_number = Some(value);
        self
    }

    pub fn with_generation_number(mut self, value: u16) -> Self {
        self.generation_number = Some(value);
        self
    }

    pub fn push(&mut self, value: impl Into<PdfObject>) {
        self.values.push(value.into());
    }

    pub fn encode(&self) -> Result<Vec<u8>, PdfError> {
        let mut arr = vec![];
        arr.push(b'[');
        arr.push(b' ');
        for pdf_object in &self.values {
            arr.extend(pdf_object.encode_as_value()?);
            arr.push(b' ');
        }
        arr.push(b']');

        Ok(arr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NumberType;
    use crate::objects::boolean::PdfBooleanObject;
    use crate::objects::name::PdfNameObject;
    use crate::objects::number::PdfNumberObject;
    use crate::objects::reference::PdfReferenceObject;

    #[test]
    fn encode_empty_array() {
        let arr = PdfArrayObject::new();
        assert_eq!(arr.encode().unwrap(), b"[ ]");
    }

    #[test]
    fn encode_single_element() {
        let mut arr = PdfArrayObject::new();
        arr.push(PdfNumberObject::new(NumberType::Integer(42)));
        assert_eq!(arr.encode().unwrap(), b"[ 42 ]");
    }

    #[test]
    fn encode_mixed_elements() {
        let mut arr = PdfArrayObject::new();
        arr.push(PdfNumberObject::new(NumberType::Integer(549)));
        arr.push(PdfNumberObject::new(NumberType::Real(3.14)));
        arr.push(PdfBooleanObject::new(false));
        assert_eq!(arr.encode().unwrap(), b"[ 549 3.14 false ]");
    }

    #[test]
    fn encode_with_name() {
        let mut arr = PdfArrayObject::new();
        arr.push(PdfNameObject::new("SomeName"));
        assert_eq!(arr.encode().unwrap(), b"[ /SomeName ]");
    }

    #[test]
    fn encode_with_indirect_reference() {
        let mut arr = PdfArrayObject::new();
        arr.push(PdfReferenceObject::new(ObjectNumber::new(10)));
        assert_eq!(arr.encode().unwrap(), b"[ 10 0 R  ]");
    }
}
