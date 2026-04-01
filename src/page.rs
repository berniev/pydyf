/// Page: (pdf dictionary)
///
/// The attributes of a page, organized into various categories (e.g., Font, ColorSpace, Pattern)
///
///     A page object may not have children.
///
/// Inh = Can be inherited from parent pageTree heirarchy, which satisfies Reqd.
///
/// ====================  ===  ====  ===  ================  ===================================
/// Entry Key             Ver  Reqd  Inh  Type              Value
/// ====================  ===  ====  ===  ================  ===================================
/// Type                       Reqd       name              "Page"
/// Parent                     Reqd       dictionary        indirect reference
/// LastModified               *          date              * Reqd if PieceInfo
/// Resources                  Reqd  Inh  dictionary
/// MediaBox                   Reqd  Inh  rectangle
///
/// Annots                     Opt        array
/// Contents                   Opt        stream or array
/// CropBox                    Opt   Inh  rectangle
/// Rotate                     Opt   Inh  integer
/// Thumb                      Opt        stream
/// Trans                      Opt        dictionary
///
/// B                     1.1  Opt        array
/// Dur                   1.1  Opt        number
///
/// AA                    1.2  Opt        dictionary
///
/// ArtBox                1.3  Opt        rectangle
/// BleedBox              1.3  Opt        rectangle
/// ID byte               1.3  Opt        string
/// PieceInfo             1.3  Opt        dictionary
/// PZ                    1.3  Opt        number
/// SeparationInfo        1.3  Opt        dictionary
/// StructParents         1.3  *          integer          Reqd if struct content items
/// TrimBox               1.3  Opt        rectangle
///
/// BoxColorInfo          1.4  Opt        dictionary
/// Group                 1.4  Opt        dictionary
/// Metadata              1.4  Opt        stream
///
/// PresSteps             1.5  Opt        dictionary
/// Tabs                  1.5  Opt        name
/// TemplateInstantiated  1.5  Opt        name
///
/// UserUnit              1.6  Opt        number
/// VP                    1.6  Opt        dictionary

//==============================================================================================//

/// PageTree: (pdf dictionary)
///
/// Nodes:
///
/// ========  ==========  =====  ===  ===============================================
/// Name      PdfObjType  Reqd   Inh  Value
/// ========  ==========  =====  ===  ===============================================
/// Type      Name        Reqd        "Pages"
/// Parent    Indirect    Reqd*       Parent PageTree. * Not allowed in root node.
/// Kids      Array       Reqd        Indirect references to descendant pages
/// Count     Integer     Reqd        Number of descendant pages
///
/// Resources Dictionary  Opt    Inh
/// MediaBox  Rectangle   Opt    Inh
/// CropBox   Rectangle   Opt    Inh
/// Rotate    Integer     Opt    Inh
///
use std::fmt;
use std::iter::Sum;

use crate::objects::pdf_object::Pdf;
pub use crate::page_size::PageSize;
use crate::{PdfArrayObject, PdfDictionaryObject, PdfObject};

//--------------------------- ObjectId ---------------------------//

#[derive(Clone, Debug, Default)]
pub struct ObjectId(u64);

impl From<u64> for ObjectId {
    fn from(value: u64) -> Self {
        ObjectId(value)
    }
}

impl From<usize> for ObjectId {
    fn from(value: usize) -> Self {
        ObjectId(value as u64)
    }
}

impl From<ObjectId> for u64 {
    fn from(object_num: ObjectId) -> u64 {
        object_num.0
    }
}

impl Sum for ObjectId {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        ObjectId(iter.map(|id| id.0).sum())
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

//--------------------------- Page ---------------------------//

pub fn make_page() -> PdfDictionaryObject {
    let tree = PdfDictionaryObject::new().typed("Page");

    tree
}

//--------------------------- PageTree -------------------------//

pub fn make_page_tree() -> PdfDictionaryObject {
    let mut tree = PdfDictionaryObject::new().typed("Pages");
    tree.add("Kids", Pdf::array(PdfArrayObject::new()));
    tree.add("Count", Pdf::num(0));

    tree
}

fn add_page_to_tree(mut page: PdfDictionaryObject, mut tree: PdfDictionaryObject) {
    if !page.contains_key("Resources") {
        let res: Option<&Box<dyn PdfObject>> = tree.get("Resources");
        page.add(
            "Resources",
            if res.is_some() {
                Pdf::dict(Some(res))
            } else {
                Pdf::dict(PdfDictionaryObject::new())
            },
        );
    }

    if let Some(obj) = tree.get_mut("Kids") {
        if let Some(array) = obj.as_any_mut().downcast_mut::<PdfArrayObject>() {
            array.push(Pdf::dict(page));
        }
    }
}

