use crate::encoding::f64_to_pdf_string;
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

    pub fn increment_object_number(&mut self) -> ObjectNumber {
        self.object_number = self.object_number.next();
        self.object_number
    }
}

//--------------------------- WriteObject -------------------------//

pub(crate) fn write_indirect_object(
    object_number: ObjectNumber,
    encoded: Vec<u8>,
    xref: &mut XRefOps,
    file: &mut File,
) -> Result<(), PdfError> {
    let mut vec = vec![];
    let offset = file.stream_position()?;
    vec.extend(object_number.to_string().as_bytes());
    vec.extend(b" 0 obj\n");
    vec.extend(encoded);
    vec.extend(b"endobj\n\n");
    file.write_all(&vec)?;

    let xref_ent = XRefEntry::new(
        object_number,
        offset,
        ObjectStatus::InUse,
        Generation::Normal,
    );
    xref.add_entry(xref_ent);
    Ok(())
}

//--------------------------- PdfEncode -------------------------//

pub trait PdfEncode: std::any::Any {
    fn pdf_encode(&mut self, version: Version) -> Result<Vec<u8>, PdfError>;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl PdfEncode for &'static str {
    fn pdf_encode(&mut self, version: Version) -> Result<Vec<u8>, PdfError> {
        PdfStringObject::new(self).encode(version)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

//--------------------------- object -------------------------//

macro_rules! encode_pdf_object {
    ($ty:ty) => {
        impl PdfEncode for $ty {
            fn pdf_encode(&mut self, version: Version) -> Result<Vec<u8>, PdfError> {
                self.encode(version)
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };
}
encode_pdf_object!(PdfBooleanObject);
encode_pdf_object!(PdfNameObject);
encode_pdf_object!(PdfNullObject);
encode_pdf_object!(PdfNumberObject);
encode_pdf_object!(PdfStringObject);
encode_pdf_object!(PdfArrayObject);
encode_pdf_object!(PdfDictionaryObject);
encode_pdf_object!(PdfReferenceObject);
encode_pdf_object!(PdfStreamObject);

//--------------------------- primitive -------------------------//

macro_rules! encode_primitive {
    ($ty:ty, $encode_expr:expr) => {
        impl PdfEncode for $ty {
            fn pdf_encode(&mut self, _version: Version) -> Result<Vec<u8>, PdfError> {
                $encode_expr(self)
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };
}

encode_primitive!(bool, |s: &mut bool| Ok(if *s {
    b"true".to_vec()
} else {
    b"false".to_vec()
}));

encode_primitive!(usize, |s: &mut usize| Ok(s.to_string().into_bytes()));
encode_primitive!(i64, |s: &mut i64| Ok(s.to_string().into_bytes()));
encode_primitive!(i32, |s: &mut i32| Ok(s.to_string().into_bytes()));
encode_primitive!(f64, |s: &mut f64| Ok(f64_to_pdf_string(*s).into_bytes()));
encode_primitive!(f32, |s: &mut f32| Ok(
    f64_to_pdf_string(*s as f64).into_bytes()
));
encode_primitive!(u64, |s: &mut u64| Ok(s.to_string().into_bytes()));
encode_primitive!(u32, |s: &mut u32| Ok(s.to_string().into_bytes()));
encode_primitive!(u8, |s: &mut u8| Ok(s.to_string().into_bytes()));
encode_primitive!(String, |s: &mut String| Ok(s.as_bytes().to_vec()));
encode_primitive!(ObjectNumber, |s: &mut ObjectNumber| Ok(s
    .value()
    .to_string()
    .into_bytes()));

//--------------------------- deduced objects -------------------------//

impl From<usize> for Box<dyn PdfEncode> { fn from(v: usize) -> Self { Box::new(PdfNumberObject::from(v)) } }
impl From<i64> for Box<dyn PdfEncode> { fn from(v: i64) -> Self { Box::new(PdfNumberObject::from(v)) } }
impl From<f64> for Box<dyn PdfEncode> { fn from(v: f64) -> Self { Box::new(PdfNumberObject::from(v)) } }
impl From<u64> for Box<dyn PdfEncode> { fn from(v: u64) -> Self { Box::new(PdfNumberObject::from(v)) } }
impl From<i32> for Box<dyn PdfEncode> { fn from(v: i32) -> Self { Box::new(PdfNumberObject::from(v)) } }
impl From<f32> for Box<dyn PdfEncode> { fn from(v: f32) -> Self { Box::new(PdfNumberObject::from(v)) } }
impl From<u32> for Box<dyn PdfEncode> { fn from(v: u32) -> Self { Box::new(PdfNumberObject::from(v)) } }
impl From<u8> for Box<dyn PdfEncode> { fn from(v: u8) -> Self { Box::new(PdfNumberObject::from(v as  i64)) } }
impl From<bool> for Box<dyn PdfEncode> { fn from(v: bool) -> Self { Box::new(PdfBooleanObject::new(v)) } }

impl From<PdfNameObject> for Box<dyn PdfEncode> {
    fn from(v: PdfNameObject) -> Self { Box::new(v) }
}

impl From<PdfStringObject> for Box<dyn PdfEncode> {
    fn from(v: PdfStringObject) -> Self { Box::new(v) }
}

impl From<PdfArrayObject> for Box<dyn PdfEncode> {
    fn from(v: PdfArrayObject) -> Self { Box::new(v) }
}

impl From<PdfDictionaryObject> for Box<dyn PdfEncode> {
    fn from(v: PdfDictionaryObject) -> Self { Box::new(v) }
}

impl From<PdfStreamObject> for Box<dyn PdfEncode> {
    fn from(v: PdfStreamObject) -> Self { Box::new(v) }
}

impl From<PdfReferenceObject> for Box<dyn PdfEncode> {
    fn from(v: PdfReferenceObject) -> Self { Box::new(v) }
}

impl From<PdfNullObject> for Box<dyn PdfEncode> {
    fn from(v: PdfNullObject) -> Self { Box::new(v) }
}

pub fn serialize_object(value:  &mut Box<dyn PdfEncode>, version: Version, xref: &mut XRefOps, file: &mut File) -> Result<(), PdfError> {
    if let Some(stream) = value.as_any_mut().downcast_mut::<PdfStreamObject>() {
        stream.serialize(version, xref, file)?;
    } else if let Some(arr) = value.as_any_mut().downcast_mut::<PdfArrayObject>() {
        arr.serialize(version, xref, file)?;
    } else if let Some(dict) = value.as_any_mut().downcast_mut::<PdfDictionaryObject>() {
        dict.serialize(version, xref, file)?;
    }
    else if let Some(ref_obj) = value.as_any_mut().downcast_mut::<PdfReferenceObject>() {
        ref_obj.serialize(version, xref, file)?;
    }
    else if let Some(string) = value.as_any_mut().downcast_mut::<PdfStringObject>() {
        string.serialize(version, xref, file)?;
    }
    else if let Some(number) = value.as_any_mut().downcast_mut::<PdfNumberObject>() {
        number.serialize(version, xref, file)?;
    }
    else if let Some(bool) = value.as_any_mut().downcast_mut::<PdfBooleanObject>() {
        bool.serialize(version, xref, file)?;
    }
    else if let Some(name) = value.as_any_mut().downcast_mut::<PdfNameObject>() {
        name.serialize(version, xref, file)?;
    }   
    else if let Some(null) = value.as_any_mut().downcast_mut::<PdfNullObject>() {
        null.serialize(version, xref, file)?;
    }

    Ok(())
}
