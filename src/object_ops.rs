use crate::generation::Generation;
pub(crate) use crate::object_number::ObjectNumber;
use crate::objects::pdf_number::PdfNumberObject;
use crate::version::Version;
use crate::xref_ops::{ObjectStatus, XRefEntry, XRefOps};
use crate::{
    PdfArrayObject, PdfBooleanObject, PdfDictionaryObject, PdfError, PdfNameObject, PdfNullObject,
    PdfReferenceObject, PdfStreamObject, PdfStringObject,
};
use std::fs::File;
use std::io::{Seek, Write};

//--------------------------- ObjectOps -------------------------//

pub struct ObjectOps {
    object_number: ObjectNumber,
}

impl ObjectOps {
    pub fn new() -> Self {
        Self {
            // 0 is in xref table as 'free'. is gen# 65535, else 0 for new
            object_number: ObjectNumber::new(0),
        }
    }

    pub fn object_number(&self) -> ObjectNumber {
        self.object_number
    }

    pub fn next_object_number(&mut self) -> ObjectNumber {
        self.object_number = self.object_number.next();
        self.object_number
    }
}

impl From<ObjectNumber> for Box<dyn PdfObject> {
    fn from(v: ObjectNumber) -> Self {
        Box::new(PdfReferenceObject::new(v))
    }
}

//--------------------------- PdfObject -------------------------//

pub trait PdfObject: std::any::Any {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn serialize_object(
        &mut self,
        version: Version,
        xref: &mut XRefOps,
        file: &mut File,
    ) -> Result<(), PdfError>;
}

macro_rules! pdf_object {
    ($ty:ty) => {
        impl PdfObject for $ty {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }

            fn serialize_object(
                &mut self,
                version: Version,
                xref: &mut XRefOps,
                file: &mut File,
            ) -> Result<(), PdfError> {
                self.serialize(version, xref, file)
            }
        }
    };
}
pdf_object!(PdfBooleanObject);
pdf_object!(PdfNameObject);
pdf_object!(PdfNullObject);
pdf_object!(PdfNumberObject);
pdf_object!(PdfStringObject);
pdf_object!(PdfArrayObject);
pdf_object!(PdfDictionaryObject);
pdf_object!(PdfReferenceObject);
pdf_object!(PdfStreamObject);

//--------------------------- deduced objects -------------------------//

impl From<usize> for Box<dyn PdfObject> {
    fn from(v: usize) -> Self {
        Box::new(PdfNumberObject::from(v))
    }
}
impl From<i64> for Box<dyn PdfObject> {
    fn from(v: i64) -> Self {
        Box::new(PdfNumberObject::from(v))
    }
}
impl From<f64> for Box<dyn PdfObject> {
    fn from(v: f64) -> Self {
        Box::new(PdfNumberObject::from(v))
    }
}
impl From<u64> for Box<dyn PdfObject> {
    fn from(v: u64) -> Self {
        Box::new(PdfNumberObject::from(v))
    }
}
impl From<i32> for Box<dyn PdfObject> {
    fn from(v: i32) -> Self {
        Box::new(PdfNumberObject::from(v))
    }
}
impl From<f32> for Box<dyn PdfObject> {
    fn from(v: f32) -> Self {
        Box::new(PdfNumberObject::from(v))
    }
}
impl From<u32> for Box<dyn PdfObject> {
    fn from(v: u32) -> Self {
        Box::new(PdfNumberObject::from(v))
    }
}
impl From<u8> for Box<dyn PdfObject> {
    fn from(v: u8) -> Self {
        Box::new(PdfNumberObject::from(v as i64))
    }
}

//--------------------------- Encode -------------------------//

pub trait Encode {
    fn encode(&self, _version: Version) -> Result<Vec<u8>, PdfError> {
        Ok(Vec::new())
    }
}

//--------------------------- Serialize -------------------------//

pub trait Serialize: Encode {
    fn serialize(
        &mut self,
        version: Version,
        _xref: &mut XRefOps,
        file: &mut File,
    ) -> Result<(), PdfError> {
        file.write(&*self.encode(version)?)?;

        Ok(())
    }

    fn indirect_xref_entry(&self, object_number: ObjectNumber, offset: u64) -> XRefEntry {
        XRefEntry::new(
            object_number,
            offset,
            ObjectStatus::InUse,
            Generation::Normal,
        )
    }

    fn try_indirect_start(
        &self,
        xref: &mut XRefOps,
        file: &mut File,
        object_number: Option<ObjectNumber>,
    ) -> Result<(), PdfError> {
        if object_number.is_some() {
            let object_number = object_number.unwrap();
            xref.add_entry(self.indirect_xref_entry(object_number, file.stream_position()?));
            file.write(format!("{} 0 obj", object_number.to_string()).as_bytes())?;
        }

        Ok(())
    }

    fn try_indirect_end(
        &self,
        file: &mut File,
        object_number: Option<ObjectNumber>,
    ) -> Result<(), PdfError> {
        if object_number.is_some() {
            file.write("endobj\n\n".to_string().as_bytes())?;
        }

        Ok(())
    }
}
//--------------------------- tests -------------------------//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_true() {
        let obj = PdfBooleanObject::new(true);
        assert_eq!(obj.encode(Version::V1_5).expect("REASON"), b"true");
    }

    #[test]
    fn encode_false() {
        let obj = PdfBooleanObject::new(false);
        assert_eq!(obj.encode(Version::V1_5).expect("REASON"), b"false");
    }

    #[test]
    fn encode_after_set() {
        let mut obj = PdfBooleanObject::new(true);
        obj.set(false);
        assert_eq!(obj.encode(Version::V1_5).expect("REASON"), b"false");
    }
}
