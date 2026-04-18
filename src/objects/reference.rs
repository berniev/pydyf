use crate::PdfError;

#[derive(Clone)]
pub enum HostType {
    Standard { generation_number: u16 }, // offset from start of file
    ObjectStream { stream_obj_num: usize }, // v1.5+, obj num of containing ObjStm
}

#[derive(Clone)]
pub struct PdfReferenceObject {
    host_type: HostType,
    pub object_number: Option<u64>,
    pub generation_number: Option<u16>,
}

impl PdfReferenceObject {
    pub fn new(obj_num: u64) -> Self {
        Self {
            host_type: HostType::Standard {
                generation_number: 0,
            },
            object_number: Some(obj_num),
            generation_number: None,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn with_object_number(mut self, value: u64) -> Self {
        self.object_number = Some(value);
        self
    }

    pub fn with_generation_number(mut self, value: u16) -> Self {
        self.generation_number = Some(value); 
        self
    }

    pub fn encode(&self) -> Result<Vec<u8>, PdfError> {
        let gen_num = match &self.host_type {
            HostType::Standard { generation_number } => *generation_number,
            HostType::ObjectStream { .. } => 0,
        };
        let mut vec: Vec<u8> = vec![];
        vec.extend(self.object_number.unwrap().to_string().into_bytes());
        vec.push(b' ');
        vec.extend(gen_num.to_string().into_bytes());
        vec.extend(" R ".as_bytes());

        Ok(vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_reference() {
        let obj = PdfReferenceObject::new(5);
        assert_eq!(obj.encode().unwrap(), b"5 0 R ");
    }

    #[test]
    fn encode_reference_object_one() {
        let obj = PdfReferenceObject::new(1);
        assert_eq!(obj.encode().unwrap(), b"1 0 R ");
    }

    #[test]
    fn encode_reference_large_number() {
        let obj = PdfReferenceObject::new(999);
        assert_eq!(obj.encode().unwrap(), b"999 0 R ");
    }
}