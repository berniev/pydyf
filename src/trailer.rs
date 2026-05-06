/*
Trailer entries

=======  ==========  =====  =====================================================================
Key      Type        Reqd   Value
=======  ==========  =====  =====================================================================
Size     Number      Reqd   The number of objects in the file.
Root     Object      Reqd   Indirect Ref. The object that is the root of the object hierarchy.
Info     Object      Opt    A dictionary that contains information about the document.
ID       Array       Reqd*  If Encrypt entry present, else opt, but recommended.
                            A two-element array that uniquely identifies the document.
Encrypt  Dictionary  Reqd*  If doc is encrypted. Specifies how the document is encrypted.
*/
use crate::encryption_ops::{
    EncryptionConfig, bytes_to_pdf_hex_string, compute_data_hash, compute_encryption_values,
};
use crate::object_ops::{ObjectNumber, PdfObject};
use crate::version::Version;
use crate::xref_ops::XRefOps;
use crate::{PdfArrayObject, PdfDictionaryObject, PdfError};
use std::fs::File;
use std::io::Write;

pub struct Trailer {
    dictionary: PdfDictionaryObject,
}

impl Trailer {
    pub fn new(
        last_object_number: ObjectNumber,
        catalog_object_number: ObjectNumber,
    ) -> Result<Self, PdfError> {
        let mut dictionary = PdfDictionaryObject::new();
        dictionary.add("Size", last_object_number.value() + 1)?;
        dictionary.add("Root", catalog_object_number)?;

        Ok(Self{dictionary})
    }

    pub fn encrypted(&mut self, config: &EncryptionConfig) -> Result<&mut Self, PdfError> {
        let (_hash_hex, file_id_bytes) = compute_data_hash(&[]);
        let vals = compute_encryption_values(config, &file_id_bytes);

        // Build /Encrypt dictionary
        let mut encrypt_dict = PdfDictionaryObject::new();
        encrypt_dict.add("Filter", PdfObject::name("Standard"))?;
        encrypt_dict.add("V", 1_i64)?;
        encrypt_dict.add("R", 2_i64)?;
        encrypt_dict.add("O", bytes_to_pdf_hex_string(&vals.o_value))?;
        encrypt_dict.add("U", bytes_to_pdf_hex_string(&vals.u_value))?;
        encrypt_dict.add("P", vals.permissions as i64)?;
        self.dictionary.add("Encrypt", encrypt_dict)?;

        // Build /ID array
        let id_hex = bytes_to_pdf_hex_string(&file_id_bytes);
        let mut id_array = PdfArrayObject::new();
        id_array.push(id_hex.clone());
        id_array.push(id_hex);
        self.dictionary.add("ID", id_array)?;

        Ok(self)
    }

    pub fn serialize(
        &mut self,
        version: Version,
        xref: &XRefOps,
        file: &mut File,
    ) -> Result<(), PdfError> {
        let mut bytes: Vec<u8> = vec![];
        bytes.extend(b"\ntrailer\n");
        bytes.extend(self.dictionary.encode(version)?);
        bytes.extend(format!("startxref\n{}\n%%EOF\n", xref.position).as_bytes());

        file.write_all(&bytes)?;

        Ok(())
    }
}
