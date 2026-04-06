pub fn encode_pdf_string(string: &str) -> String {
    if string.is_ascii() {
        encode_ascii(string)
    } else {
        encode_non_ascii(string)
    }
}

/// escape and wrap in parentheses
fn encode_ascii(string: &str) -> String {
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
fn encode_non_ascii(string: &str) -> String {
    let mut hex_content = String::with_capacity(string.len() * 4 + 4);
    for ch in string.encode_utf16() {
        hex_content.push_str(&format!("{:04X}", ch)); // {:04X} = "4 digits, hex, uppercase"
    }
    let bom = 0xFEFF;

    format!("{bom}<{hex_content}>")
}
