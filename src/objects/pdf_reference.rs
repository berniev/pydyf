use crate::object_ops::ObjectNumber;

#[derive(Clone)]
pub enum HostType {
    Standard { generation_number: u16 }, // offset from start of file
    Stream { stream_obj_num: usize },    // v1.5+, obj num of containing ObjStm
}

#[derive(Clone)]
pub struct PdfReferenceObject {
    pub(crate) host_type: HostType,
    pub(crate) object_number: Option<ObjectNumber>,
}

impl PdfReferenceObject {
    pub fn new(obj_num: ObjectNumber) -> Self {
        Self {
            host_type: HostType::Standard {
                generation_number: 0,
            },
            object_number: Some(obj_num),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object_ops::Encode;
    use crate::version::Version;

    #[test]
    fn encode_reference() {
        let obj = PdfReferenceObject::new(ObjectNumber::new(5));
        assert_eq!(obj.encode(Version::V1_5).expect("Reason"), b"5 0 R");
    }

    #[test]
    fn encode_reference_object_one() {
        let obj = PdfReferenceObject::new(ObjectNumber::new(1));
        assert_eq!(obj.encode(Version::V1_5).expect("Reason"), b"1 0 R");
    }

    #[test]
    fn encode_reference_large_number() {
        let obj = PdfReferenceObject::new(ObjectNumber::new(999));
        assert_eq!(obj.encode(Version::V1_5).expect("Reason"), b"999 0 R");
    }
}
