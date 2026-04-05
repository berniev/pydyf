use crate::objects::pdf_object::PdfObj;
use crate::PdfDictionaryObject;

pub fn add_font_resources(mut next_num_func: impl FnMut() -> u64) -> u64 {
    let mut resources_dict = PdfDictionaryObject::new();
    let next_num = next_num_func();
    let fonts_dict = PdfDictionaryObject::new().with_object_number(next_num);
    resources_dict.add("Font", PdfObj::dict(fonts_dict));

    next_num
}
