use crate::pdf_version::PdfVersion;

pub struct Header {
    version: PdfVersion,
}

impl Header {
    pub fn new() -> Self {
        Header {
            version: PdfVersion::Auto,
        }
    }
    
    pub fn set_version(&mut self, version: PdfVersion) {
        self.version = version;
    }

    pub fn version(&self) -> PdfVersion {
        self.version
    }

    pub fn serialise(&self) -> Vec<u8> {
        let mut arr :Vec<u8> = vec![];
        arr.extend("%PDF-".to_string().as_bytes());
        arr.extend(self.version.as_str().as_bytes());
        arr.extend("\r\nâãÏÓ\r\n".as_bytes());
        
        arr
    }
}
