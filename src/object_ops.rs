use crate::generation::Generation;
pub(crate) use crate::object_number::ObjectNumber;
use crate::objects::pdf_number::PdfNumberObject;
use crate::objects::pdf_reference::HostType;
use crate::version::Version;
use crate::xref_ops::{ObjectStatus, XRefEntry, XRefOps};
use crate::{
    CompressionMethod, PdfArrayObject, PdfBooleanObject, PdfDictionaryObject, PdfError,
    PdfNameObject, PdfNullObject, PdfNumberType, PdfReferenceObject, PdfStreamObject,
    PdfStringObject,
};
use flate2::Compression;
use flate2::write::ZlibEncoder;
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

//--------------------------- PdfObject -------------------------//

pub trait PdfObject: std::any::Any {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

//--------------------------- object -------------------------//

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

//--------------------------- serialize -------------------------//

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

pub trait Encode {
    fn encode(&self, _version: Version) -> Result<Vec<u8>, PdfError> {
        Ok(Vec::new())
    }
}

impl Encode for PdfArrayObject {}
impl Encode for PdfDictionaryObject {}
impl Encode for PdfStreamObject {}

impl Encode for PdfNameObject {
    fn encode(&self, _version: Version) -> Result<Vec<u8>, PdfError> {
        // all #'s will be encoded
        const HEX_CHARS: &[u8] = b"0123456789ABCDEF";
        let mut result: Vec<u8> = vec![b'/'];
        for &byte in &self.value {
            if byte == b'#' || !(0x21..=0x7E).contains(&byte) {
                result.push(b'#');
                result.push(HEX_CHARS[(byte >> 4) as usize]);
                result.push(HEX_CHARS[(byte & 0xF) as usize]);
            } else {
                if byte != 0x00 {
                    result.push(byte); // silently strip nulls
                }
            }
        }

        Ok(result)
    }
}

impl Encode for PdfStringObject {
    fn encode(&self, version: Version) -> Result<Vec<u8>, PdfError> {
        Ok(crate::objects::pdf_string::encode_text_string(
            &*self.value,
            version,
        ))
    }
}

impl Encode for PdfNumberObject {
    fn encode(&self, _version: Version) -> Result<Vec<u8>, PdfError> {
        let mut arr = vec![];
        arr = match self.value {
            PdfNumberType::Integer(i) => Vec::from(&*i.to_string().into_bytes()),
            PdfNumberType::Real(f) => {
                Vec::from(
                    &*format!("{:.4}", f) // use a reasonable precision
                        .trim_end_matches('0')
                        .trim_end_matches('.')
                        .to_string()
                        .into_bytes(),
                )
            }
        };

        Ok(arr)
    }
}

impl Encode for PdfBooleanObject {
    fn encode(&self, _version: Version) -> Result<Vec<u8>, PdfError> {
        Ok(if self.value {
            Vec::from(b"true")
        } else {
            Vec::from(b"false")
        })
    }
}

impl Encode for PdfNullObject {
    fn encode(&self, _version: Version) -> Result<Vec<u8>, PdfError> {
        Ok("null".to_string().into_bytes())
    }
}

impl Encode for PdfReferenceObject {
    fn encode(&self, _version: Version) -> Result<Vec<u8>, PdfError> {
        let gen_num = match &self.host_type {
            HostType::Standard { generation_number } => *generation_number,
            HostType::Stream { .. } => 0,
        };
        let mut vec: Vec<u8> = vec![];
        vec.extend(self.object_number.unwrap().to_string().into_bytes());
        vec.push(b' ');
        vec.extend(gen_num.to_string().into_bytes());
        vec.extend(" R".as_bytes());

        Ok(vec)
    }
}

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

fn try_indirect_start(
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

fn try_indirect_end(file: &mut File, object_number: Option<ObjectNumber>) -> Result<(), PdfError> {
    if object_number.is_some() {
        file.write("endobj\n\n".to_string().as_bytes())?;
    }

    Ok(())
}

impl Serialize for PdfReferenceObject {}
impl Serialize for PdfStringObject {}
impl Serialize for PdfNumberObject {}
impl Serialize for PdfBooleanObject {}
impl Serialize for PdfNameObject {}
impl Serialize for PdfNullObject {}

impl Serialize for PdfDictionaryObject {
    fn serialize(
        &mut self,
        version: Version,
        xref: &mut XRefOps,
        file: &mut File,
    ) -> Result<(), PdfError> {
        try_indirect_start(xref, file, self.object_number)?;

        file.write(b" <<\n")?;

        for (name, pdf_object) in &mut self.entries {
            let mut name_obj = PdfNameObject::new(name);
            name_obj.serialize(version, xref, file)?;
            file.write(b" ")?;
            serialize_pdf_object(pdf_object, version, xref, file)?;
            file.write(b"\n")?;
        }

        file.write(b">>\n")?;

        try_indirect_end(file, self.object_number)?;

        Ok(())
    }
}

impl Serialize for PdfStreamObject {
    fn serialize(
        &mut self,
        version: Version,
        xref: &mut XRefOps,
        file: &mut File,
    ) -> Result<(), PdfError> {
        try_indirect_start(xref, file, Some(self.object_number))?;

        let stream_bytes: Vec<u8> = match self.compression_method {
            CompressionMethod::None => self.content.clone(),
            CompressionMethod::Flate => {
                let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
                encoder.write(&*self.content)?;
                encoder.finish()?
            }
        };

        self.dict.add("Length", stream_bytes.len())?;
        self.dict.serialize(version, xref, file)?; // dict is direct object (no object number)

        file.write(b"stream\n")?;
        file.write(&*stream_bytes)?;
        file.write(b"\nendstream\n")?;

        try_indirect_end(file, Some(self.object_number))?;

        Ok(())
    }
}

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
