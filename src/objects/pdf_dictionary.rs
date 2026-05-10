use crate::object_ops::{serialize_object, write_indirect_object, ObjectNumber, PdfEncode};
use crate::objects::pdf_number::PdfNumberObject;
use crate::version::Version;
use crate::xref_ops::XRefOps;
use crate::{PdfArrayObject, PdfError, PdfNameObject, PdfStringObject};
use std::fs::File;

/// Spec:
/// Dictionary:
///     An associative table containing pairs of objects, the first object being a name object
///     serving as the key and the second object serving as the value and may be any kind of object
///     including another dictionary.
/// Entries:
///     The entries in a dictionary represent an associative table and as such shall be unordered
///     even though an arbitrary order may be imposed upon them when written in a file. That
///     ordering shall be ignored.
///
///     Multiple entries in the same dictionary shall not have the same key.
///     A dictionary shall be written as a sequence of key-value pairs enclosed in double angle
///     brackets (<< … >>) (using LESS-THAN SIGNs (3Ch) and GREATER-THAN SIGNs (3Eh)).
///     The value of a Type entry shall be either defined in this standard or a registered name.
///         name "Type"    Opt
///         name "Subtype" Opt (requires Type)
///

pub struct PdfDictionaryObject {
    pub(crate) values: Vec<(String, Box<dyn PdfEncode>)>,
    pub(crate) object_number: Option<ObjectNumber>,
    pub(crate) children: Vec<PdfDictionaryObject>, // for page tree
}

impl PdfDictionaryObject {
    pub fn new() -> Self {
        Self {
            values: vec![],
            object_number: None,
            children: vec![],
        }
    }

    pub(crate) fn typed(mut self, name: &str) -> Result<Self, PdfError> {
        self.add("Type", PdfNameObject::new(name));

        Ok(self)
    }

    pub fn with_object_number(mut self, value: ObjectNumber) -> Self {
        self.object_number = Some(value);
        self
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    pub fn get(&self, key: &str) -> Option<&Box<dyn PdfEncode>> {
        self.values
            .iter()
            .find_map(|(k, v)| if k == key { Some(v) } else { None })
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Box<dyn PdfEncode>> {
        self.values
            .iter_mut()
            .find_map(|(k, v)| if k == key { Some(v) } else { None })
    }

    pub fn push_to_array(
        &mut self,
        key: &str,
        value: impl PdfEncode + 'static,
    ) -> Result<(), PdfError> {
        if let Some(entry) = self.get_mut(key) {
            if let Some(arr) = entry.as_any_mut().downcast_mut::<PdfArrayObject>() {
                arr.values.push(Box::new(value));
                Ok(())
            } else {
                Err(PdfError::StructureError(format!(
                    "Key '{}' is not an array",
                    key
                )))
            }
        } else {
            Err(PdfError::StructureError(format!("Key '{}' not found", key)))
        }
    }

    fn require(&self, key: &str) -> Result<&Box<dyn PdfEncode>, PdfError> {
        self.get(key)
            .ok_or_else(|| PdfError::StructureError(format!("Key '{}' not found", key)))
    }

    pub fn get_integer(&self, key: &str) -> Result<i64, PdfError> {
        let value = self.require(key)?;
        value
            .as_any()
            .downcast_ref::<PdfNumberObject>()
            .ok_or_else(|| PdfError::StructureError(format!("Key '{}' is not a number", key)))
            .map(|n| n.as_int())
    }

    pub fn get_string(&self, key: &str) -> Result<&str, PdfError> {
        let value = self.require(key)?;
        value
            .as_any()
            .downcast_ref::<PdfStringObject>()
            .ok_or_else(|| PdfError::StructureError(format!("Key '{}' is not a string", key)))
            .map(|n| n.value())
    }

    pub fn get_name(&self, key: &str) -> Result<&Vec<u8>, PdfError> {
        let value = self.require(key)?;
        value
            .as_any()
            .downcast_ref::<PdfNameObject>()
            .ok_or_else(|| PdfError::StructureError(format!("Key '{}' is not a name", key)))
            .map(|n| n.as_vec())
    }

    pub fn get_dict(&self, key: &str) -> Result<&PdfDictionaryObject, PdfError> {
        let value = self.require(key)?;
        value
            .as_any()
            .downcast_ref::<PdfDictionaryObject>()
            .ok_or_else(|| PdfError::StructureError(format!("Key '{}' is not a dictionary", key)))
            .map(|n| n)
    }

    pub fn update_or_add(&mut self, key: &str, value: impl Into<Box<dyn PdfEncode>>) {
        if let Some(existing) = self.values.iter_mut().find(|(k, _)| k == key) {
            existing.1 = value.into();
        } else {
            self.values.push((key.to_string(), value.into()));
        }
    }

    pub fn add(&mut self, key: &str, value: impl Into<Box<dyn PdfEncode>>) {
        self.values.push((key.to_string(), value.into()));
    }

    pub fn del(&mut self, key: &str) -> Result<(), PdfError> {
        if let Some(index) = self.values.iter().position(|(name, _)| name == key) {
            self.values.remove(index);
            Ok(())
        } else {
            Err(PdfError::StructureError(format!("Key '{}' not found", key)))
        }
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

        for (_name, value) in &mut self.values {
            serialize_object(value, version, xref, file)?;
        }

        for child in &mut self.children {
            child.serialize(version, xref, file)?;
        }

        Ok(())
    }

    fn encode(&mut self, version: Version) -> Result<Vec<u8>, PdfError> {
        let mut bytes: Vec<u8> = vec![];
        bytes.extend(b"<<\n");
        for (name, pdf_object) in &mut self.values {
            let mut name_obj = PdfNameObject::new(name);
            bytes.extend(name_obj.pdf_encode(version)?);
            bytes.push(b' ');
            bytes.extend(pdf_object.pdf_encode(version)?);
            bytes.extend(b"\n");
        }
        bytes.extend(b">>\n");

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionary_methods() {
        let mut dict = PdfDictionaryObject::new();
        assert!(dict.is_empty());
        assert_eq!(dict.len(), 0);

        dict.add("Key1", PdfNameObject::new("Value1"));
        assert!(!dict.is_empty());
        assert_eq!(dict.len(), 1);
        assert!(dict.contains_key("Key1"));
        assert!(!dict.contains_key("Key2"));

        dict.add("Key2", PdfNameObject::new("Value2"));
        assert_eq!(dict.len(), 2);
        assert!(dict.contains_key("Key2"));
    }

    #[test]
    fn encode_empty_dictionary() {
        let mut dict = PdfDictionaryObject::new();
        assert_eq!(dict.encode(Version::V1_5).unwrap(), b"<<\n>>\n");
    }

    #[test]
    fn encode_single_entry() {
        let mut dict = PdfDictionaryObject::new();
        dict.add("Type", PdfNameObject::new("Catalog"));
        let output = String::from_utf8(dict.encode(Version::V1_5).unwrap()).unwrap();
        assert!(output.starts_with("<<\n"));
        assert!(output.contains("/Type /Catalog"));
        assert!(output.ends_with(">>\n"));
    }

    #[test]
    fn encode_multiple_entries() {
        let mut dict = PdfDictionaryObject::new();
        dict.add("Type", PdfNameObject::new("Page"));
        dict.add("Count", 3i64);
        let output = String::from_utf8(dict.encode(Version::V1_5).unwrap()).unwrap();
        assert!(output.contains("/Type /Page"));
        assert!(output.contains("/Count 3"));
    }

    #[test]
    fn encode_with_boolean_value() {
        let mut dict = PdfDictionaryObject::new();
        dict.add("Visible", true);
        let output = String::from_utf8(dict.encode(Version::V1_5).unwrap()).unwrap();
        assert!(output.contains("/Visible true"));
    }

    #[test]
    fn encode_with_indirect_reference() {
        let mut dict = PdfDictionaryObject::new();
        dict.add("Pages", 2);
        let enc = dict.encode(Version::V1_5).expect("encoding failed");
        assert_eq!(enc, b"/Pages 2 0 R");
        let output = String::from_utf8(enc).expect("decoding failed");
        assert!(output.contains("/Pages 2 0 R"));
    }
}
