//--------------------------- HostType --------------------------//

#[derive(Clone)]
pub enum HostType {
    Standard { generation_number: u16 }, // offset from start of file
    ObjectStream { stream_obj_num: usize }, // v1.5+, obj num of containing ObjStm
}

//-------------------------- PdfIndirectObject ----------------------//

/*pub struct PdfIndirectObject {
    pub host_type: HostType,

    pub byte_offset: usize,
}

impl PdfIndirectObject {
    pub fn new_standard() -> Self {
        Self {
            host_type: HostType::Standard {
                generation_number: 0,
            },
            byte_offset: 0,
        }
    }

    pub fn new_in_obj_stream(stream_obj_num: usize) -> Self {
        Self {
            host_type: HostType::ObjectStream { stream_obj_num },
            byte_offset: 0,
        }
    }

    pub fn serialise(host_type: HostType, object_number: u64) -> Result<Vec<u8>, PdfError> {
        match host_type {
            HostType::Standard { generation_number } => {
                let mut vec: Vec<u8> = vec![];
                vec.extend(object_number.to_string().into_bytes());
                vec.push(b' ');
                vec.extend(generation_number.to_string().into_bytes());
                vec.extend(b"obj\n");
                vec.extend(object_being_wrapped.serialise()?);
                vec.extend(b"\nendobj\n");

                Ok(vec)
            }
            HostType::ObjectStream { .. } => Ok(object_being_wrapped.serialise()?),
        }
    }
}
*/