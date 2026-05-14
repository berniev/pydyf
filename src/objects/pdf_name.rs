#[derive(PartialEq)]
pub struct PdfNameObject {
    pub(crate) value: Vec<u8>,
}

impl PdfNameObject {
    pub fn new(value: impl AsRef<[u8]>) -> Self {
        let value = value.as_ref().to_vec();
        Self {
            value: Self::fix(value),
        }
    }

    pub fn value(&self) -> &Vec<u8> {
        &self.value
    }

    // nb: all #'s will be encoded
    fn fix(vec: Vec<u8>) -> Vec<u8> {
        const HEX_CHARS: &[u8] = b"0123456789ABCDEF";
        let mut result = vec![];
        for &byte in &vec {
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
        result
    }
}

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
