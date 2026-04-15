use crate::{PdfArrayObject, PdfDictionaryObject, PdfStreamObject};
use crate::objects::pdf_object::PdfObj;

pub enum MaskSubType {
    Luminosity,
    Alpha,
}

impl MaskSubType {
    pub fn as_str(&self) -> &str {
        match self {
            MaskSubType::Luminosity => "Luminosity",
            MaskSubType::Alpha => "Alpha",
        }
    }
}

pub struct SoftMask {
    dictionary: PdfDictionaryObject,
}

impl SoftMask {
    pub fn new(sub_type: MaskSubType, stream: PdfStreamObject) -> Self {
        let mut msk = SoftMask {
            dictionary: PdfDictionaryObject::new(),
        };
        msk.dictionary
            .add("S", PdfObj::make_name_obj(sub_type.as_str()));
        msk.dictionary.add("G", stream);

        msk
    }

    pub fn typed(mut self) -> Self {
        self.dictionary.add("Type", PdfObj::make_name_obj("Mask"));

        self
    }

    pub fn with_backdrop(mut self, backdrop: PdfArrayObject) -> Self {
        self.dictionary.add("BG", backdrop);

        self
    }

    pub fn with_function(mut self, function: PdfDictionaryObject) -> Self {
        self.dictionary.add("TR", function);

        self
    }

    pub fn with_function_identity(mut self) -> Self {
        self.dictionary.add("TR", PdfObj::make_name_obj("Identity"));

        self
    }
}
