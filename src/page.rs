use crate::objects::base::IndirectReference;
use crate::objects::metadata::PdfMetadata;
use crate::{DictionaryObject, PdfObject};
use std::sync::Arc;

//--------------------------- Page ---------------------------//

pub struct Page {
    pub metadata: PdfMetadata,
    pub page_size: PageSize,
    pub dict: DictionaryObject, // The core dictionary (/Type /Page, etc)
    pub contents: Option<Arc<dyn PdfObject>>, // The page's actual drawing commands
}

impl Default for Page {
    fn default() -> Self {
        Page {
            metadata: PdfMetadata::default(),
            page_size: PageSize::default(),
            dict: DictionaryObject::typed("Page"),
            contents: None,
        }
    }
}

impl Page {
    pub fn new(size: PageSize) -> Self {
        Self {
            page_size: size,
            ..Default::default()
        }
    }

    pub fn set_parent(&mut self, parent_id: usize) {
        self.dict.set(
            "Parent",
            Arc::new(IndirectReference {
                metadata: PdfMetadata::default(),
                id: parent_id,
            }),
        );
    }

    pub fn set_size(&mut self, size: PageSize) {
        self.page_size = size;
    }

    pub fn set_contents(&mut self, contents: Vec<u8>) {
        self.contents = contents;
    }

    pub fn set_resources(&mut self, resources: DictionaryObject) {
        self.resources = Some(resources);
    }

    pub fn to_dictionary(&self) -> DictionaryObject {
        let mut values = self.other.clone();
        values.insert("Type".to_string(), b"/Page".to_vec());
        if let Some(size) = self.size {
            values.insert("MediaBox".to_string(), size.to_mediabox());
        }
        if !self.contents.is_empty() {
            values.insert("Contents".to_string(), self.contents.clone());
        }
        if let Some(resources) = &self.resources {
            values.insert("Resources".to_string(), resources.data());
        }

        let mut dict = DictionaryObject::new(Some(values));
        dict.metadata = self.metadata.clone();
        dict
    }
}

