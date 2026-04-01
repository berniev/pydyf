/// String Object:
///     Consists of a series of bytes (unsigned integer values in the range 0 to 255) and the bytes
///     are not integer objects, but are stored in a more compact form
///
use crate::{PdfError, PdfObject};
use std::any::Any;

//--------------------------- PdfStringObject----------------------//

pub struct PdfStringObject {
    value: String,
}

impl PdfStringObject {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl PdfObject for PdfStringObject {
    fn serialise(&mut self) -> Result<Vec<u8>, PdfError> {
        Ok(self.value.as_bytes().to_vec())
    }

    fn is_indirect_by_default(&self) -> bool {
        false
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub fn encode_pdf_string(string: &str) -> String {
    if string.is_ascii() {
        encode_ascii(string)
    } else {
        encode_non_ascii(string)
    }
}

/// escape and wrap in parentheses
pub fn encode_ascii(string: &str) -> String {
    let mut result = String::with_capacity(string.len() + 4);
    result.push('(');
    for ch in string.chars() {
        if matches!(ch, '\\' | '(' | ')') {
            result.push('\\');
        }
        result.push(ch);
    }
    result.push(')');

    result
}

/// UTF-16BE encode with BOM and hex-encode
pub fn encode_non_ascii(string: &str) -> String {
    let mut hex_content = String::with_capacity(string.len() * 4 + 4);
    for ch in string.encode_utf16() {
        hex_content.push_str(&format!("{:04X}", ch)); // {:04X} = "4 digits, hex, uppercase"
    }

    format!("FEFF<{}>", hex_content)
}
