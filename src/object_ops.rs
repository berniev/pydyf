use crate::generation::Generation;
pub(crate) use crate::object_number::ObjectNumber;
use crate::objects::pdf_number::PdfNumberObject;
use crate::version::Version;
use crate::xref_ops::{ObjectStatus, XRefEntry, XRefOps};
use crate::{
    PdfArrayObject, PdfBooleanObject, PdfDictionaryObject, PdfError,
    PdfNameObject, PdfNullObject, PdfReferenceObject, PdfStreamObject,
    PdfStringObject,
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

//--------------------------- Free functions   -------------------------//

pub fn indirect_xref_entry(object_number: ObjectNumber, offset: u64) -> XRefEntry {
    XRefEntry::new(
        object_number,
        offset,
        ObjectStatus::InUse,
        Generation::Normal,
    )
}

pub(crate) fn try_indirect_start(
    xref: &mut XRefOps,
    file: &mut File,
    object_number: Option<ObjectNumber>,
) -> Result<(), PdfError> {
    if object_number.is_some() {
        let object_number = object_number.unwrap();
        xref.add_entry(indirect_xref_entry(object_number, file.stream_position()?));
        file.write(format!("{} 0 obj", object_number.to_string()).as_bytes())?;
    }

    Ok(())
}

pub(crate) fn try_indirect_end(file: &mut File, object_number: Option<ObjectNumber>) -> Result<(), PdfError> {
    if object_number.is_some() {
        file.write("endobj\n\n".to_string().as_bytes())?;
    }

    Ok(())
}

//--------------------------- PdfObject -------------------------//

pub trait PdfObject: std::any::Any {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
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

//--------------------------- primitive -------------------------//

macro_rules! primitive {
    ($ty:ty, $encode_expr:expr) => {
        impl PdfObject for $ty {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };
}

primitive!(bool, |s: &mut bool| Ok(if *s {
    b"true".to_vec()
} else {
    b"false".to_vec()
}));

primitive!(usize, |s: &mut usize| Ok(s.to_string().into_bytes()));
primitive!(i64, |s: &mut i64| Ok(s.to_string().into_bytes()));
primitive!(i32, |s: &mut i32| Ok(s.to_string().into_bytes()));
primitive!(f64, |s: &mut f64| Ok(f64_to_pdf_string(*s).into_bytes()));
primitive!(f32, |s: &mut f32| Ok(
    f64_to_pdf_string(*s as f64).into_bytes()
));
primitive!(u64, |s: &mut u64| Ok(s.to_string().into_bytes()));
primitive!(u32, |s: &mut u32| Ok(s.to_string().into_bytes()));
primitive!(u8, |s: &mut u8| Ok(s.to_string().into_bytes()));
primitive!(String, |s: &mut String| Ok(s.as_bytes().to_vec()));
primitive!(ObjectNumber, |s: &mut ObjectNumber| Ok(s
    .value()
    .to_string()
    .into_bytes()));

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
impl From<bool> for Box<dyn PdfObject> {
    fn from(v: bool) -> Self {
        Box::new(PdfBooleanObject::new(v))
    }
}
impl From<ObjectNumber> for Box<dyn PdfObject> {
    fn from(v: ObjectNumber) -> Self {
        Box::new(PdfReferenceObject::new(v))
    }
}

impl From<PdfNameObject> for Box<dyn PdfObject> {
    fn from(v: PdfNameObject) -> Self {
        Box::new(v)
    }
}

impl From<PdfStringObject> for Box<dyn PdfObject> {
    fn from(v: PdfStringObject) -> Self {
        Box::new(v)
    }
}

impl From<PdfArrayObject> for Box<dyn PdfObject> {
    fn from(v: PdfArrayObject) -> Self {
        Box::new(v)
    }
}

impl From<PdfDictionaryObject> for Box<dyn PdfObject> {
    fn from(v: PdfDictionaryObject) -> Self {
        Box::new(v)
    }
}

impl From<PdfStreamObject> for Box<dyn PdfObject> {
    fn from(v: PdfStreamObject) -> Self {
        Box::new(v)
    }
}

impl From<PdfReferenceObject> for Box<dyn PdfObject> {
    fn from(v: PdfReferenceObject) -> Self {
        Box::new(v)
    }
}

impl From<PdfNullObject> for Box<dyn PdfObject> {
    fn from(v: PdfNullObject) -> Self {
        Box::new(v)
    }
}

impl From<PdfNumberObject> for Box<dyn PdfObject> {
    fn from(v: PdfNumberObject) -> Self {
        Box::new(v)
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
}

pub fn serialize_pdf_object(
    value: &mut Box<dyn PdfObject>,
    version: Version,
    xref: &mut XRefOps,
    file: &mut File,
) -> Result<(), PdfError> {
    if let Some(stream) = value.as_any_mut().downcast_mut::<PdfStreamObject>() {
        stream.serialize(version, xref, file)?;
    } else if let Some(arr) = value.as_any_mut().downcast_mut::<PdfArrayObject>() {
        arr.serialize(version, xref, file)?;
    } else if let Some(dict) = value.as_any_mut().downcast_mut::<PdfDictionaryObject>() {
        dict.serialize(version, xref, file)?;
    } else if let Some(ref_obj) = value.as_any_mut().downcast_mut::<PdfReferenceObject>() {
        ref_obj.serialize(version, xref, file)?;
    } else if let Some(string) = value.as_any_mut().downcast_mut::<PdfStringObject>() {
        string.serialize(version, xref, file)?;
    } else if let Some(number) = value.as_any_mut().downcast_mut::<PdfNumberObject>() {
        number.serialize(version, xref, file)?;
    } else if let Some(bool) = value.as_any_mut().downcast_mut::<PdfBooleanObject>() {
        bool.serialize(version, xref, file)?;
    } else if let Some(name) = value.as_any_mut().downcast_mut::<PdfNameObject>() {
        name.serialize(version, xref, file)?;
    } else if let Some(null) = value.as_any_mut().downcast_mut::<PdfNullObject>() {
        null.serialize(version, xref, file)?;
    }

    Ok(())
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
