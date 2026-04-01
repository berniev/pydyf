/// PDF Spec:
///
/// Indirect is a wrapper, not a peer variant
///
/// Any object in a PDF file may be labelled as an indirect object. This gives the object a
/// unique object identifier by which other objects can refer to it (for example, as an
/// element of an array or as the value of a dictionary entry).
///
/// An object identifier shall consist of two parts:
/// - A positive integer object number. Indirect objects may be numbered sequentially
///   within a PDF file, but this is not required; object numbers may be assigned in any
///   arbitrary order.
/// - A non-negative integer generation number. In a newly created file, all indirect
///   objects shall have generation numbers of 0. Nonzero generation numbers will be
///   introduced when the file is later updated.
///
///       Example: {obj_num} {gen_num} obj {object} endobj
///
/// Together, the combination of an object number and a generation number shall uniquely
/// identify an indirect object.
///
use crate::{PdfError, PdfObject};

//--------------------------- HostType --------------------------//

pub enum HostType {
    Standard { generation_number: u16 },
    ObjectStream { stream_obj_num: usize }, // v1.5+, stream_obj_num is obj num of containing ObjStm
}

//-------------------------- PdfIndirectObject ----------------------//

pub struct PdfIndirectObject {
    pub object_being_wrapped: PdfObject,
    pub host_type: HostType,

    /// HostType       byte_offset from
    /// =============  ================
    /// Standard       start of file
    /// ObjectStream  `First` in the object stream
    pub byte_offset: usize,
}

impl PdfIndirectObject {
    pub fn new_standard(object_being_wrapped: PdfObject) -> Self {
        Self {
            object_being_wrapped,
            host_type: HostType::Standard {
                generation_number: 0,
            },
            byte_offset: 0,
        }
    }

    pub fn new_in_obj_stream(object_being_wrapped: PdfObject, stream_obj_num: usize) -> Self {
        Self {
            object_being_wrapped,
            host_type: HostType::ObjectStream { stream_obj_num },
            byte_offset: 0,
        }
    }

/*    pub fn reference(&self) -> Vec<u8> {
        let gen_num = match &self.host_type {
            HostType::Standard { generation_number } => *generation_number,
            HostType::ObjectStream { .. } => 0,
        };
        format!("{} {} R", self.obj_num, gen_num).into_bytes()
    }
*/}

pub fn serialise_indirect_object(
    mut object_being_wrapped: PdfObject,
    host_type: HostType,
    object_number: u64,
) -> Result<Vec<u8>, PdfError> {
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

/*fn default_is_indirect() -> bool {
    true
}
*/