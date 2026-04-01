/// File Structure
///
/// =====================  =====================================================================
/// Header                 One line identifying pdf version
/// Body                   The objects that make up the document
/// Cross-Reference Table  Information about the __indirect__ objects in the file
/// Trailer                Location of the xref tbl and of certain special objects in the file body
/// ============================================================================================
///

/**
space lines are optional
```
%PDF-1.4                    ← header
%âãÏÓ                       ← comment in the body, not required nowadays but spec does say 'shall'
1 0 obj                     ← first actual body object
...
endobj
...

xref                        ← cross-reference table
0 9
0000000000 65535 f\r\n
...

trailer                     ← trailer
<<
  /Size 9
  /Root 1 0 R
>>
startxref
1234                        ← byte offset of xref
%%EOF
```
*/
/*
How to determine if an object will be indirect or direct?

Three criteria, in order of priority:
1. Spec mandates it — some objects must be indirect. Streams are always indirect. Certain
   dictionary entries like /Pages and /Outlines are required to be indirect references. No choice.
2. Shared — if the same object is referenced from multiple places, it must be indirect so multiple
   references can point to it by number. A font used on every page is the classic example.
3. Size / complexity — large or complex objects (page dictionaries, images, fonts) are better
   as indirect so the cross-reference table enables random access to them without parsing the
   whole file. Small trivial values (a boolean, a name, a short string) are better inline as
   direct objects — the overhead of an indirect object isn't worth it.

In practice for a write-once writer: if in doubt, make it indirect.
The cost is just an xref entry.

so maybe for starters we:
    indirect: stream, dict, array.
    direct  : bool, name, string, number, null. (always part of an indirect object)

    So the concrete rule is: if the object has a /Type entry, it's almost certainly indirect.
    Typed objects are named, standalone PDF entities. Untyped objects are supporting data
    embedded in their parent.
*/
use std::io::Write;

use crate::cross_reference_table::{CrossRefTable, CrossReferenceEntry};
use crate::file_identifier::FileIdentifierMode;
use crate::fonts::Fonts;
use crate::header::Header;
use crate::objects::pdf_object::Pdf;
use crate::page::make_page_tree;
use crate::pdf_version::PdfVersion;
use crate::trailer::Trailer;
use crate::writer::{CompressedStrategy, LegacyStrategy, PdfWriter};
use crate::{PdfDictionaryObject, PdfObject};
use crate::body::Body;

//--------------------------- PDF -------------------------//

pub struct PdfFile {
    header: Header,
    body: Body,
    cross_reference_table: CrossRefTable,
    trailer: Trailer,

    xref_position: Option<usize>,

    pages_root: PdfDictionaryObject, // catalog /Pages entry must point to this
}

impl PdfFile {
    pub fn new() -> Self {
        PdfFile {
            header: Header::new(),
            body: Body::new(),
            cross_reference_table: CrossRefTable::new(),
            trailer: Trailer::new(),

            xref_position: None,
            pages_root: make_page_tree(),
        }
    }

    pub fn version(mut self, version: PdfVersion) -> Self {
        self.header.set_version(version);

        self
    }

    // all objects added here are to be indirect
    pub fn add_object(&mut self, obj: Box<dyn PdfObject>) -> usize {
        let object_number = self.body.next_num();
        self.cross_reference_table.add_entry(CrossReferenceEntry {
            object_number,
            object_status: Default::default(), // in use
            offset_or_next_free: 0,
            generation: 0,
        });



        0
    }

    fn write_common(&mut self) {
        let resources_number = self.add_font_resources();
        self.initialize_catalog();
    }

    pub fn write_legacy<W: Write>(
        &mut self,
        output: W,
        id_mode: FileIdentifierMode,
    ) -> std::io::Result<()> {
        self.write_common();
        PdfWriter::new(output, LegacyStrategy::default(), id_mode).perform(self)
    }

    pub fn write_compressed<W: Write>(
        &mut self,
        output: W,
        id_mode: FileIdentifierMode,
    ) -> std::io::Result<()> {
        self.write_common();
        PdfWriter::new(output, CompressedStrategy::default(), id_mode).perform(self)
    }

    pub fn add_font_resources(&mut self) -> usize {
        let mut resources_dict = PdfDictionaryObject::new();
        resources_dict.add("Font", Pdf::dict(Fonts::get_standard_fonts_dict()));

        self.indirect_pdf_objects.push(resources_dict.boxed());

        let resources_number = self.allocate_object_id();
        resources_dict.metadata.object_identifier = Some(resources_number);

        resources_number
    }

    pub fn initialize_catalog(&mut self) {
        let pages_id = self.pages_root.metadata.object_identifier.unwrap();
        self.catalog.add("Pages", pages_id);

        let catalog_copy = self.catalog.clone();
        self.indirect_pdf_objects.push(Box::new(catalog_copy));
    }
}
