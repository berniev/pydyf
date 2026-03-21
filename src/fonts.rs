use crate::{DictionaryObject, NameObject};

pub(crate) struct Fonts {}

impl Fonts {
    pub(crate) fn get_standard_fonts_dict() -> DictionaryObject {
        let mut font_dict = DictionaryObject::new(None);
        let fonts = [
            ("Helvetica", "Type1"),
            ("Helvetica-Bold", "Type1"),
            ("Courier", "Type1"),
        ];

        for (name, subtype) in fonts {
            let mut f = DictionaryObject::typed("Font");
            f.set("Subtype", NameObject::make_pdf_obj(subtype));
            f.set("BaseFont", NameObject::make_pdf_obj(name));
            font_dict.set(name, DictionaryObject::make_pdf_obj(f.values));
        }

        font_dict
    }

    #[allow(dead_code)]
    fn get_standard_fonts() -> String {
        let fonts = [
            ("Helvetica", "Type1"),
            ("Helvetica-Bold", "Type1"),
            ("Courier", "Type1"),
        ];
        format!(
            "<<{}>>",
            fonts
                .into_iter()
                .map(|(name, subtype)| format!(
                    " /{name} << /Type /Font /Subtype /{subtype} /BaseFont /{name} >>"
                ))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
