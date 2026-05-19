use crate::object_ops::{Encode, PdfObject, Serialize};
use crate::PdfError;
use crate::version::Version;

#[derive(Clone)]
pub struct PdfNullObject {}

impl PdfNullObject {
    pub fn new() -> Self {
        Self {}
    }
}

impl PdfNullObject {}

impl Encode for PdfNullObject {
    fn encode(&self, _version: Version) -> Result<Vec<u8>, PdfError> {
        Ok("null".to_string().into_bytes())
    }
}

impl Serialize for PdfNullObject {}

impl From<PdfNullObject> for Box<dyn PdfObject> {
    fn from(v: PdfNullObject) -> Self {
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
    fn encode_null() {
        let obj = PdfNullObject::new();
        assert_eq!(obj.encode(Version::V1_5).expect("REASON"), b"null");
    }
}
