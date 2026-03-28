use crate::objects::array::PdfArrayObject;
use crate::objects::boolean::PdfBooleanObject;
use crate::objects::dictionary::PdfDictionaryObject;
use crate::objects::name::PdfNameObject;
use crate::objects::number::PdfNumberObject;
use crate::objects::stream::PdfStreamObject;
use crate::objects::string::PdfStringObject;

enum PdfObjectType {
    Array(PdfArrayObject),
    Boolean(PdfBooleanObject),
    Dictionary(PdfDictionaryObject),
    Name(PdfNameObject), // ascii
    NameTree(),
    Number(PdfNumberObject), // integer and real
    Stream(PdfStreamObject),
    String(PdfStringObject), // may need encoding
}

