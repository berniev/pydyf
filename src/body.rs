/*
The body of a PDF file shall consist of a sequence of indirect objects representing the contents of
a document. These indirect objects will contain the actual data (direct objects) for the document.

The objects, which are of the basic types described in 7.3, "Objects," represent components of the
document such as fonts, pages, and sampled images.

Beginning with PDF 1.5, the body can also contain object streams, each of which contains a
sequence of indirect objects;
*/
use crate::{PdfObject};

pub struct Body {
    last_object_number: u64,
}

impl Body {
    pub fn new() -> Self {
        Body {
            last_object_number: 0,
        }
    }

    pub fn add_indirect_object(&mut self, mut object: Box<dyn PdfObject>){
        self.last_object_number += 1;
        let serialised = object.serialise();
    }
    
    pub fn next_num(&mut self) ->u64{
        self.last_object_number += 1;
        
        self.last_object_number
    }
}
