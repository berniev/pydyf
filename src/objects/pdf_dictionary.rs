use std::fs::File;
use std::io::Write;
use crate::object_ops::{Encode, ObjectNumber, PdfObject, Serialize};
use crate::{PdfArrayObject, PdfError, PdfNameObject};
use crate::version::Version;
use crate::xref_ops::XRefOps;

//--------------------------- PdfDictionaryObject -------------------------//

pub struct PdfDictionaryObject {
    pub(crate) entries: Vec<(String, Box<dyn PdfObject>)>,
    pub(crate) object_number: Option<ObjectNumber>,
}

fn bad_type_error<T>(key: &str) -> PdfError {
    PdfError::StructureError(format!(
        "Key '{}' is not of type {}",
        key,
        std::any::type_name::<T>()
    ))
}

fn not_found_error(key: &str) -> PdfError {
    PdfError::StructureError(format!("Key '{}' not found", key))
}

fn duplicate_key_error(key: &str) -> PdfError {
    PdfError::StructureError(format!("Key '{}' already exists", key))
}

impl PdfDictionaryObject {
    pub fn new() -> Self {
        Self {
            entries: vec![],
            object_number: None,
        }
    }

    pub(crate) fn typed(mut self, name: &str) -> Self {
        self.add("Type", PdfNameObject::new(name)).unwrap();
        self
    }

    pub fn with_object_number(mut self, value: ObjectNumber) -> Self {
        self.object_number = Some(value);
        self
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn push_to_array(&mut self, key: &str, value: impl PdfObject) -> Result<(), PdfError> {
        let arr = self.get_t_mut::<PdfArrayObject>(key)?;
        arr.pdf_objects.push(Box::new(value));
        Ok(())
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    fn get(&self, key: &str) -> Option<&Box<dyn PdfObject>> {
        self.entries
            .iter()
            .find_map(|(k, v)| if k == key { Some(v) } else { None })
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Box<dyn PdfObject>> {
        self.entries
            .iter_mut()
            .find_map(|(k, v)| if k == key { Some(v) } else { None })
    }

    fn require_key(&self, key: &str) -> Result<&dyn PdfObject, PdfError> {
        self.get(key)
            .map(|v| v.as_ref())
            .ok_or_else(|| not_found_error(key))
    }

    fn require_key_mut(&mut self, key: &str) -> Result<&mut dyn PdfObject, PdfError> {
        self.get_mut(key)
            .map(|v| v.as_mut())
            .ok_or_else(|| not_found_error(key))
    }

    pub fn get_t<T: PdfObject>(&self, key: &str) -> Result<&T, PdfError> {
        self.require_key(key)?.as_any()
            .downcast_ref::<T>()
            .ok_or_else(|| bad_type_error::<T>(key))
    }

    pub fn get_t_mut<T: PdfObject>(&mut self, key: &str) -> Result<&mut T, PdfError> {
        self.require_key_mut(key)?.as_any_mut()
            .downcast_mut::<T>()
            .ok_or_else(|| bad_type_error::<T>(key))
    }

    pub fn add(&mut self, key: &str, value: impl Into<Box<dyn PdfObject>>) ->Result<(), PdfError>{
        if self.contains_key(key) {
            return Err(duplicate_key_error(key));
        }
        self.entries.push((key.to_string(), value.into()));
        Ok(())
    }

    pub fn update_or_add(&mut self, key: &str, value: impl Into<Box<dyn PdfObject>>) {
        if let Some(existing) = self.entries.iter_mut().find(|(k, _)| k == key) {
            existing.1 = value.into();
        } else {
            self.entries.push((key.to_string(), value.into()));
        }
    }

    pub fn update(
        &mut self,
        key: &str,
        value: impl Into<Box<dyn PdfObject>>,
    ) -> Result<(), PdfError> {
        if let Some(existing) = self.entries.iter_mut().find(|(k, _)| k == key) {
            existing.1 = value.into();
            Ok(())
        } else {
            Err(not_found_error(key))
        }
    }

    pub fn del(&mut self, key: &str) -> Result<(), PdfError> {
        if let Some(index) = self.entries.iter().position(|(name, _)| name == key) {
            self.entries.remove(index);
            Ok(())
        } else {
            Err(not_found_error(key))
        }
    }
}

impl Encode for PdfDictionaryObject {}

impl Serialize for PdfDictionaryObject {
    fn serialize(
        &mut self,
        version: Version,
        xref: &mut XRefOps,
        file: &mut File,
    ) -> Result<(), PdfError> {
        self.try_indirect_start(xref, file, self.object_number)?;

        file.write(b" <<\n")?;

        for (name, pdf_object) in &mut self.entries {
            let mut name_obj = PdfNameObject::new(name);
            name_obj.serialize(version, xref, file)?;
            file.write(b" ")?;
            pdf_object.serialize_object(version, xref, file)?;
            file.write(b"\n")?;
        }

        file.write(b">>\n")?;

        self.try_indirect_end(file, self.object_number)?;

        Ok(())
    }
}

impl From<PdfDictionaryObject> for Box<dyn PdfObject> {
    fn from(v: PdfDictionaryObject) -> Self {
        Box::new(v)
    }
}

//--------------------------- tests -------------------------//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionary_methods() {
        let mut dict = PdfDictionaryObject::new();
        assert!(dict.is_empty());
        assert_eq!(dict.len(), 0);

        dict.add("Key1", PdfNameObject::new("Value1")).unwrap();
        assert!(!dict.is_empty());
        assert_eq!(dict.len(), 1);
        assert!(dict.contains_key("Key1"));
        assert!(!dict.contains_key("Key2"));

        dict.add("Key2", PdfNameObject::new("Value2")).unwrap();
        assert_eq!(dict.len(), 2);
        assert!(dict.contains_key("Key2"));
    }
}
