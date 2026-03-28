use crate::PdfDictionaryObject;

pub(crate) struct Fonts {}

impl Fonts {
    pub(crate) fn get_standard_fonts_dict() -> PdfDictionaryObject {
        let fonts = [
            ("Helvetica", "Type1"),
            ("Helvetica-Bold", "Type1"),
            ("Courier", "Type1"),
        ];

        let mut fonts_dict = PdfDictionaryObject::new();
        for (name, subtype) in fonts {
            let mut dict = PdfDictionaryObject::new().typed("Font");
            dict.add_name("Subtype", subtype);
            dict.add_name("BaseFont", name);
            
            fonts_dict.add_pdf_dict(name, dict);
        }

        fonts_dict
    }
}
