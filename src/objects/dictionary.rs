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
///
use crate::{
    NumberType, PdfArrayObject, PdfBooleanObject, PdfIndirectObject, PdfNameObject,
    PdfNumberObject, PdfObject,
};

//--------------------------- PdfDictionaryObject----------------------//

pub struct PdfDictionaryObject {
    pub(crate) values: Vec<(PdfNameObject, Box<dyn PdfObject>)>,
}

impl PdfDictionaryObject {
    pub fn new() -> Self {
        Self { values: vec![] }
    }

    pub(crate) fn typed(mut self, name: &str) -> Self {
        self.set(name, self.make_name(name).boxed());

        self
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.values.iter().any(|(k, _)| k.value == key)
    }

    fn make_name(&self, name: &str) -> PdfNameObject {
        PdfNameObject::new(name)
    }

    fn set(&mut self, key: &str, object: Box<dyn PdfObject>) {
        // todo: duplicates not allowed
        let k_obj = self.make_name(key);
        self.values.push((k_obj, object));
    }

    //--------------------------- Add Methods -----------------------//

    pub fn add_string(&mut self, key: &str, value: String) {
        self.set(key, self.make_name(&value).boxed());
    }

    pub fn add_name(&mut self, key: &str, value: &str) {
        self.set(key, PdfNameObject::new(value).boxed());
    }

    pub fn add_indirect_norm(&mut self, key: &str, value: usize, obj: Box<dyn PdfObject>) {
        self.set(key, PdfIndirectObject::new_standard(value, obj).boxed());
    }

     pub fn add_indirect_in_stream(
        &mut self,
        key: &str,
        value: usize,
        obj: Box<dyn PdfObject>,
        num: usize,
    ) {
        self.set(
            key,
            PdfIndirectObject::new_in_obj_stream(value, obj, num).boxed(),
        );
    }

    pub fn add_bool(&mut self, key: &str, value: bool) {
        self.set(key, PdfBooleanObject::new(value).boxed());
    }

    pub fn add_number(&mut self, key: &str, value: impl Into<NumberType>) {
        self.set(key, PdfNumberObject::new(value.into()).boxed());
    }

    pub fn add_indirect_ref(&mut self, key: &str, value: PdfIndirectReference) {
        self.set(key, value.boxed());
    }

    /// param: PdfArrayObject
    pub fn add_pdf_array(&mut self, key: &str, array: PdfArrayObject) {
        self.set(key, array.boxed());
    }

    /// param: PdfDictionaryObject
    pub fn add_pdf_dict(&mut self, key: &str, value: PdfDictionaryObject) {
        self.set(key, value.boxed());
    }
}

impl PdfObject for PdfDictionaryObject {
    fn serialise(&mut self) -> Vec<u8> {
        let mut arr = vec![];
        arr.extend(b"<<");
        for (pdf_name_obj, pdf_object) in &mut self.values {
            arr.extend(pdf_name_obj.serialise());
            arr.push(b' ');
            arr.extend(pdf_object.serialise());
        }
        arr.extend(b">>");

        arr
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

        dict.add_name("Key1", "Value1");
        assert!(!dict.is_empty());
        assert_eq!(dict.len(), 1);
        assert!(dict.contains_key("Key1"));
        assert!(!dict.contains_key("Key2"));

        dict.add_name("Key2", "Value2");
        assert_eq!(dict.len(), 2);
        assert!(dict.contains_key("Key2"));
    }
}
