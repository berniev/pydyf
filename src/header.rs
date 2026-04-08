use crate::PdfError;
use crate::cross_reference_table::CrossRefTable;
use crate::version::Version;
use std::fs::File;
use std::io::Write;

pub struct Header {
    version: Version,
}

impl Header {
    pub fn new() -> Self {
        Header {
            version: Version::default(),
        }
    }

    pub fn set_version(&mut self, version: Version) {
        self.version = version;
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn serialise(&self, xref: &mut CrossRefTable, file: &mut File) -> Result<(), PdfError> {
        let mut arr: Vec<u8> = vec![];
        arr.extend(b"%PDF-");
        arr.extend(self.version.as_bytes());
        arr.extend(b"\r\n");
        arr.extend("âãÏÓ\r\n".as_bytes());

        file.write_all(&arr).map_err(PdfError::Io)?;
        xref.xref_position += arr.len() as u64;

        Ok(())
    }
}
