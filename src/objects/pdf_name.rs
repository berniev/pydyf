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
