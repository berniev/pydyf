use crate::objects::number::PdfNumberObject;
use crate::{
    NumberType, PdfArrayObject, PdfBooleanObject, PdfDictionaryObject, PdfError, PdfNameObject,
    PdfNullObject, PdfStreamObject, PdfStringObject,
};
use crate::cross_reference_table::ObjectStatus;
use crate::generation::Generation;
//--------------------------- Pdf -------------------------//

pub struct Pdf {}

impl Pdf {
    pub fn array(value: PdfArrayObject) -> PdfObject {
        PdfObject::Array(value)
    }

    pub fn bool(value: bool) -> PdfObject {
        PdfObject::Boolean(PdfBooleanObject::new(value))
    }

    pub fn dict(value: PdfDictionaryObject) -> PdfObject {
        PdfObject::Dictionary(value)
    }

    pub fn name(value: &str) -> PdfObject {
        PdfObject::Name(PdfNameObject::new(value))
    }

    pub fn null() -> PdfObject {
        PdfObject::Null(PdfNullObject::new())
    }

    pub fn num(value: impl Into<NumberType>) -> PdfObject {
        PdfObject::Number(PdfNumberObject::new(value.into()))
    }

    pub fn num_or_null<T: Into<NumberType>>(value: Option<T>) -> PdfObject {
        match value {
            Some(v) => Pdf::num(v),
            None => Pdf::null(),
        }
    }

    pub fn stream(value: PdfStreamObject) -> PdfObject {
        PdfObject::Stream(value)
    }

    pub fn string(value: &str) -> PdfObject {
        PdfObject::String(PdfStringObject::new(value))
    }
}
/*
Is it referenced from more than one place? → indirect (shared fonts, images, etc.)
Does anything need to refer to it by object number? → indirect (e.g. a page in the Kids array)
Is it a stream? → indirect (spec mandates it)
Everything else → direct
*/

// Tracks where an object ended up after serialisation — not intrinsic to the object itself
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SerialLocation {
    pub offset: usize,
    pub status: ObjectStatus, // free or inuse
}

// The PDF spec identity of an __ indirect __ object (§7.3.10)
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectId {
    pub number: usize,          // 0 is root. 1 is first object
    pub generation: Generation, // for obj#0 is 65535, else is 0 for new objects
}

// Indirect is a wrapper, not a peer variant
// Example: {id} {gen} obj {object} endobj
#[allow(dead_code)]
struct IndirectObject {
    pub id: ObjectId,
    pub location: Option<SerialLocation>,
    //pub inner: PdfObjectType, // owns the direct object
}

//--------------------------- PdfObject -------------------------//

// IfReqd:
//   referenced from multiple places (shared fonts, images, etc.), or
//   referenced by object number (e.g. a page in the Kids array)
pub enum Indirect {
    Always,
    Never,
    IfReferenced,
}

#[derive(Clone)]
pub enum PdfObject {
    Array(PdfArrayObject),
    Boolean(PdfBooleanObject),
    Dictionary(PdfDictionaryObject),
    Name(PdfNameObject),
    Null(PdfNullObject),
    Number(PdfNumberObject),
    Stream(PdfStreamObject),
    String(PdfStringObject),
}

impl PdfObject {
    pub fn serialise(&mut self) -> Result<Vec<u8>, PdfError> {
        match self {
            PdfObject::Array(a) => a.serialise(),
            PdfObject::Boolean(b) => b.serialise(),
            PdfObject::Dictionary(d) => d.serialise(),
            PdfObject::Name(na) => na.serialise(),
            PdfObject::Null(nu) => nu.serialise(),
            PdfObject::Number(m) => m.serialise(),
            PdfObject::Stream(s) => s.serialise(),
            PdfObject::String(sg) => sg.serialise(),
        }
    }

    pub fn is_indirect_by_default(&self) -> Indirect {
        match self {
            PdfObject::Array(_) => Indirect::IfReferenced,
            PdfObject::Boolean(_) => Indirect::Never,
            PdfObject::Dictionary(_) => Indirect::IfReferenced,
            PdfObject::Name(_) => Indirect::Never,
            PdfObject::Number(_) => Indirect::Never,
            PdfObject::Null(_) => Indirect::Never,
            PdfObject::Stream(_) => Indirect::Always,
            PdfObject::String(_) => Indirect::Never,
        }
    }
}
