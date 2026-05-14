use crate::object_ops::{ObjectNumber, PdfObject};
use crate::objects::pdf_number::PdfNumberObject;
use crate::{PdfArrayObject, PdfError, PdfNameObject, PdfStringObject};

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
    pub(crate) entries: Vec<(String, Box<dyn PdfObject>)>,
    pub(crate) object_number: Option<ObjectNumber>,
}

impl PdfDictionaryObject {
    pub fn new() -> Self {
        Self {
            entries: vec![],
            object_number: None,
        }
    }

    pub(crate) fn typed(mut self, name: &str) -> Self {
    self.add("Type", PdfNameObject::new(name));

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

    pub fn push_to_array(
        &mut self,
        key: &str,
        value: impl PdfObject + 'static,
    ) -> Result<(), PdfError> {
        if let Some(entry) = self.get_mut(key) {
            if let Some(arr) = entry.as_any_mut().downcast_mut::<PdfArrayObject>() {
                arr.elements.push(Box::new(value));
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

    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    pub fn get(&self, key: &str) -> Option<&Box<dyn PdfObject>> {
        self.entries
            .iter()
            .find_map(|(k, v)| if k == key { Some(v) } else { None })
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Box<dyn PdfObject>> {
        self.entries
            .iter_mut()
            .find_map(|(k, v)| if k == key { Some(v) } else { None })
    }

    fn require_key(&self, key: &str) -> Result<&Box<dyn PdfObject>, PdfError> {
        self.get(key)
            .ok_or_else(|| PdfError::StructureError(format!("Key '{}' not found", key)))
    }

    pub fn get_t<T: PdfObject>(&self, key: &str) -> Result<&T, PdfError> {
        let value = self.require_key(key)?;
        value.as_any().downcast_ref::<T>().ok_or_else(|| {
            PdfError::StructureError(format!(
                "Key '{}' is not of type {}",
                key,
                std::any::type_name::<T>()
            ))
        })
    }

    pub fn get_integer_value(&self, key: &str) -> Result<i64, PdfError> {
        let value = self.require_key(key)?;
        value
            .as_any()
            .downcast_ref::<PdfNumberObject>()
            .ok_or_else(|| PdfError::StructureError(format!("Key '{}' is not a number", key)))
            .map(|n| n.as_int())
    }

    pub fn get_string_value(&self, key: &str) -> Result<&str, PdfError> {
        let value = self.require_key(key)?;
        value
            .as_any()
            .downcast_ref::<PdfStringObject>()
            .ok_or_else(|| PdfError::StructureError(format!("Key '{}' is not a string", key)))
            .map(|n| n.value())
    }

    pub fn get_name_value(&self, key: &str) -> Result<&Vec<u8>, PdfError> {
        let value = self.require_key(key)?;
        value
            .as_any()
            .downcast_ref::<PdfNameObject>()
            .ok_or_else(|| PdfError::StructureError(format!("Key '{}' is not a name", key)))
            .map(|n| n.value())
    }

    pub fn get_dict_value(&self, key: &str) -> Result<&PdfDictionaryObject, PdfError> {
        let value = self.require_key(key)?;
        value
            .as_any()
            .downcast_ref::<PdfDictionaryObject>()
            .ok_or_else(|| PdfError::StructureError(format!("Key '{}' is not a dictionary", key)))
    }

    pub fn get_array_value(&self, key: &str) -> Result<&PdfArrayObject, PdfError> {
        let value = self.require_key(key)?;
        value
            .as_any()
            .downcast_ref::<PdfArrayObject>()
            .ok_or_else(|| PdfError::StructureError(format!("Key '{}' is not an array", key)))
    }

    pub fn add(&mut self, key: &str, value: impl Into<Box<dyn PdfObject>>) {
        self.entries.push((key.to_string(), value.into()));
    }

    pub fn update_or_add(&mut self, key: &str, value: impl Into<Box<dyn PdfObject>>) {
        if let Some(existing) = self.entries.iter_mut().find(|(k, _)| k == key) {
            existing.1 = value.into();
        } else {
            self.entries.push((key.to_string(), value.into()));
        }
    }

    pub fn update(&mut self, key: &str, value: impl Into<Box<dyn PdfObject>>) -> Result<(), PdfError> {
        if let Some(existing) = self.entries.iter_mut().find(|(k, _)| k == key) {
            existing.1 = value.into();

            Ok(())
        } else {
            Err(PdfError::StructureError(format!("Key '{}' not found", key)))
        }
    }

    pub fn del(&mut self, key: &str) -> Result<(), PdfError> {
        if let Some(index) = self.entries.iter().position(|(name, _)| name == key) {
            self.entries.remove(index);
            Ok(())
        } else {
            Err(PdfError::StructureError(format!("Key '{}' not found", key)))
        }
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
}
