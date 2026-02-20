//! PDF String encoding functions

use regex::Regex;

const BOM_UTF16_BE: [u8; 2] = [0xFE, 0xFF];

/// Encode a string for PDF format.
/// ASCII strings are escaped and wrapped in parentheses.
/// Non-ASCII strings are UTF-16-BE encoded with BOM and hex-encoded.
pub fn encode_pdf_string(string: &str) -> Vec<u8> {
    if string.is_ascii() {
        encode_ascii(string)
    } else {
        encode_non_ascii(string)
    }
}

fn encode_ascii(string: &str) -> Vec<u8> {
    let re = Regex::new(r"([\\()])").unwrap();
    let escaped = re.replace_all(string, r"\$1");
    let mut result = b"(".to_vec();
    result.extend(escaped.as_bytes());
    result.push(b')');
    result
}

fn encode_non_ascii(string: &str) -> Vec<u8> {
    // Use UTF-16-BE with BOM
    let mut encoded = BOM_UTF16_BE.to_vec();
    for ch in string.encode_utf16() {
        encoded.extend(&ch.to_be_bytes());
    }
    let hex_string = hex::encode(&encoded);
    let mut result = b"<".to_vec();
    result.extend(hex_string.as_bytes());
    result.push(b'>');
    result
}
