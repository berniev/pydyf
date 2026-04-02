use crate::objects::indirect::HostType;
use crate::PdfError;

#[derive(Clone)]
pub struct PdfReferenceObject {
    host_type: HostType,
    pub object_number: Option<u64>,
}

impl PdfReferenceObject {
    pub fn new(obj_num: u64) -> Self {
        Self {
            host_type: HostType::Standard {
                generation_number: 0,
            },
            object_number: Some(obj_num),
        }
    }

    pub fn serialise(&self) ->  Result<Vec<u8>, PdfError> {
        let gen_num = match &self.host_type {
            HostType::Standard { generation_number } => *generation_number,
            HostType::ObjectStream { .. } => 0,
        };

        let mut vec: Vec<u8> = vec![];
        vec.extend(self.object_number.unwrap().to_string().into_bytes());
        vec.push(b' ');
        vec.extend(gen_num.to_string().into_bytes());
        vec.push(b' ');
        vec.push(b'R');

        Ok(vec)
    }
}
