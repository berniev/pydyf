#[derive(Clone)]
pub struct PdfNullObject {}

impl PdfNullObject {
    pub fn new() -> Self {
        Self {}
    }
}

impl PdfNullObject {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object_ops::Encode;
    use crate::version::Version;

    #[test]
    fn encode_null() {
        let obj = PdfNullObject::new();
        assert_eq!(obj.encode(Version::V1_5).expect("REASON"), b"null");
    }
}
