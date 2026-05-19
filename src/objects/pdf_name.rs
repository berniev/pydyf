use crate::object_ops::{Encode, PdfObject, Serialize};
use crate::PdfError;
use crate::version::Version;

#[derive(PartialEq)]
pub struct PdfNameObject {
    pub(crate) value: Vec<u8>,
}

impl PdfNameObject {
    pub fn new(value: impl AsRef<[u8]>) -> Self {
        let value = value.as_ref().to_vec();
        Self {
            value,
        }
    }
}

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

impl Serialize for PdfNameObject {}

impl From<PdfNameObject> for Box<dyn PdfObject> {
    fn from(v: PdfNameObject) -> Self {
        Box::new(v)
    }
}

//--------------------------- tests -------------------------//

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object_ops::Encode;
    use crate::version::Version;

    #[test]
    fn encode_simple_name() {
        let obj = PdfNameObject::new("Type");
        assert_eq!(
            obj.encode(Version::V1_5).expect("REASON").to_vec(),
            b"/Type"
        );
    }

    #[test]
    fn encode_longer_name() {
        let obj = PdfNameObject::new("FlateDecode");
        assert_eq!(
            obj.encode(Version::V1_5).expect("REASON").to_vec(),
            b"/FlateDecode"
        );
    }

    #[test]
    fn encode_empty_name() {
        let obj = PdfNameObject::new("");
        assert_eq!(obj.encode(Version::V1_5).expect("REASON").to_vec(), b"/");
    }
}
