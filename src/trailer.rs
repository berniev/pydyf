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
use crate::catalog::CatalogOps;
use crate::encryption_ops::{
    bytes_to_pdf_hex_string, compute_data_hash, compute_encryption_values, EncryptionConfig,
};
use crate::object_ops::ObjectOps;
use crate::objects::pdf_object::PdfObj;
use crate::xref_ops::XRefOps;
use crate::{PdfArrayObject, PdfDictionaryObject, PdfError};
use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;

pub struct Trailer {
    dictionary: PdfDictionaryObject,
}

impl Trailer {
    pub fn new(
        object_ops: Rc<RefCell<ObjectOps>>,
        catalog_ops: &CatalogOps,
    ) -> Result<Self, PdfError> {
        let mut trailer = Trailer {
            dictionary: PdfDictionaryObject::new(),
        };

        trailer
            .dictionary
            .add("Size", object_ops.borrow().last_object_number().value() + 1)?;
        trailer
            .dictionary
            .add("Root", PdfObj::reference_obj(catalog_ops.catalog_id()))?;

        Ok(trailer)
    }

    pub fn encrypted(&mut self, config: &EncryptionConfig) -> Result<&mut Self, PdfError> {
        let (_hash_hex, file_id_bytes) = compute_data_hash(&[]);
        let vals = compute_encryption_values(config, &file_id_bytes);

        // Build /Encrypt dictionary
        let mut encrypt_dict = PdfDictionaryObject::new();
        encrypt_dict.add("Filter", PdfObj::name_obj("Standard"))?;
        encrypt_dict.add("V", PdfObj::num_obj(1_i64))?;
        encrypt_dict.add("R", PdfObj::num_obj(2_i64))?;
        encrypt_dict.add("O", PdfObj::string_obj(&bytes_to_pdf_hex_string(&vals.o_value)))?;
        encrypt_dict.add("U", PdfObj::string_obj(&bytes_to_pdf_hex_string(&vals.u_value)))?;
        encrypt_dict.add("P", PdfObj::num_obj(vals.permissions as i64))?;
        self.dictionary.add("Encrypt", encrypt_dict)?;

        // Build /ID array
        let id_hex = bytes_to_pdf_hex_string(&file_id_bytes);
        let mut id_array = PdfArrayObject::new();
        id_array.push(PdfObj::string_obj(&id_hex));
        id_array.push(PdfObj::string_obj(&id_hex));
        self.dictionary.add("ID", id_array)?;

        Ok(self)
    }

    pub fn serialise(&self, xref: &XRefOps, file: &mut File) -> Result<(), PdfError> {
        let mut bytes: Vec<u8> = vec![];
        bytes.extend(b"\ntrailer\n");
        bytes.extend(self.dictionary.encode()?);
        bytes.extend(format!("startxref\n{}\n%%EOF\n", xref.position).as_bytes());

        file.write_all(&bytes)?;

        Ok(())
    }
}
