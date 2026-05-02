use crate::PdfError;
use crate::version::Version;

#[derive(Clone)]
pub struct PdfStringObject {
    pub(crate) value: String,
}

impl PdfStringObject {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, PdfError> {
        Ok(self.value.as_bytes().to_vec())
    }
}

const BOM_UTF16: [u8; 2] = [0xFE, 0xFF];
const BOM_UTF8: [u8; 3] = [0xEF, 0xBB, 0xBF];

fn encode_text_string(string: &str, version: Version) -> Vec<u8> {
    if string.is_ascii() {
        encode_ascii(string)
    } else if version >= Version::V2_2017 {
        encode_non_ascii_utf8(string)
    } else {
        encode_non_ascii_utf16(string)
    }
}

fn encode_ascii(string: &str) -> Vec<u8> {
    let mut result :Vec<u8> = vec![];
    result.push(b'(');
    for ch in string.chars() {
        if matches!(ch, '\\' | '(' | ')') {
            result.push(b'\\');
        }
        result.push(u8::try_from(ch).unwrap());
    }
    result.push(b')');

    result
}

fn encode_non_ascii_utf16(string: &str) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(2 + string.len() * 2);
    bytes.extend_from_slice(&BOM_UTF16);
    for unit in string.encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    bytes
}

fn encode_non_ascii_utf8(string: &str) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(3 + string.len());
    bytes.extend_from_slice(&BOM_UTF8);
    bytes.extend_from_slice(string.as_bytes());
    bytes
}

//--------------------------- Tests -------------------------//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_simple_string() {
        let obj = PdfStringObject::new("Hello, World!");
        assert_eq!(obj.encode().unwrap(), b"Hello, World!");
    }

    #[test]
    fn encode_empty_string() {
        let obj = PdfStringObject::new("");
        assert_eq!(obj.encode().unwrap(), b"");
    }

    #[test]
    fn encode_string_with_newline() {
        let obj = PdfStringObject::new("line1\nline2");
        assert_eq!(obj.encode().unwrap(), b"line1\nline2");
    }
}