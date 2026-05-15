use crate::PdfStreamObject;
use crate::object_ops::{ObjectNumber, ObjectOps, Serialize};
pub use crate::page_size::PageSize;
use crate::version::Version;
use crate::xref_ops::XRefOps;
use crate::{PdfArrayObject, PdfDictionaryObject, PdfError};
use std::cell::RefCell;
use std::fs::File;
use std::rc::Rc;
use crate::fonts::standard_fonts_dict;

///
/// Page dict entries:
/// ==============================================================================
/// Name                  Ver  R  I  Type      Value
/// ====================  ===  =  =  ========  ===================================
/// Type                       R     name      "Page"
/// Parent                     R     dict      indirect reference
/// LastModified               *     date      * Reqd if PieceInfo
/// Resources                  R  I  dict      Reqd if not inherited
/// MediaBox                   R  I  rect      Reqd if not inherited
///
/// Annots                     O     array
/// Contents                   O     stream or array
/// CropBox                    O  I  rect
/// Rotate                     O  I  int
/// Thumb                      O     stream
/// Trans                      O     dict
///
/// B                     1.1  O     array
/// Dur                   1.1  O     number
///
/// AA                    1.2  O     dict
///
/// ArtBox                1.3  O     rect
/// BleedBox              1.3  O     rect
/// ID byte               1.3  O     string
/// PieceInfo             1.3  O     dict
/// PZ                    1.3  O     number
/// SeparationInfo        1.3  O     dict
/// StructParents         1.3  *     int         Reqd if struct content items
/// TrimBox               1.3  O     rect
///
/// BoxColorInfo          1.4  O     dict
/// Group                 1.4  O     dict
/// Metadata              1.4  O     stream
///
/// PresSteps             1.5  O     dict
/// Tabs                  1.5  O     name
/// TemplateInstantiated  1.5  O     name
///
/// UserUnit              1.6  O     numb
/// VP                    1.6  O     dict
/// ==============================================================================
///

///
/// PageTreeNode dict entries:
/// ========  ==========  =====  ===  ===========================================
/// Name      PdfObjType  Reqd   Inh  Value
/// ========  ==========  =====  ===  ===========================================
/// Type      Name        Reqd        "Pages"
/// Parent    Indirect    Reqd*       Parent PageTree. * Not allowed in root node
/// Kids      Array       Reqd        Indirect references to descendant pages
/// Count     Integer     Reqd        Number of descendant pages
///
/// Resources Dictionary  Opt    Inh
/// MediaBox  Rectangle   Opt    Inh
/// CropBox   Rectangle   Opt    Inh
/// Rotate    Integer     Opt    Inh
/// =============================================================================
///

//--------------------------- PageFactory -------------------------

pub struct PageFactory {
    object_ops: Rc<RefCell<ObjectOps>>,
}
impl PageFactory {
    pub fn new(object_ops: Rc<RefCell<ObjectOps>>) -> Self {
        PageFactory { object_ops }
    }

    pub fn new_tree(&self) -> PageTree {
        let object_number = self.object_ops.borrow_mut().next_object_number();
        PageTree::new(object_number)
    }

    pub fn new_page(&self, content: Vec<u8>) -> Page {
        Page::new(self.object_ops.clone(), content)
    }
}

//--------------------------- PageOps -------------------------------

pub struct PageOps {
    pub root_tree: PageTree,
}
impl PageOps {
    pub fn new(object_ops: Rc<RefCell<ObjectOps>>) -> Result<Self, PdfError> {
        let mut root_tree = PageFactory::new(object_ops).new_tree();
        let mut resources = PdfDictionaryObject::new();
        resources.add("Font", standard_fonts_dict()?);
        root_tree.dictionary.add("Resources", resources);
        root_tree
            .dictionary
            .add("MediaBox", PageSize::default().as_pdf_array());

        Ok(PageOps { root_tree })
    }

    pub fn set_default_page_size(&mut self, page_size: PageSize) {
        self.root_tree
            .dictionary
            .update_or_add("MediaBox", page_size.as_pdf_array());
    }

    pub fn root_tree_mut(&mut self) -> &mut PageTree {
        &mut self.root_tree
    }

    pub fn root_tree(&self) -> &PageTree {
        &self.root_tree
    }

    pub fn serialize(
        &mut self,
        version: Version,
        xref: &mut XRefOps,
        file: &mut File,
    ) -> Result<(), PdfError> {
        let mut count: usize = 0;
        self.root_tree.update_counts(&mut count);
        self.root_tree.serialize(version, xref, file)
    }
}

//--------------------------- PageTree -------------------------------

pub struct PageTree {
    dictionary: PdfDictionaryObject,
    trees: Vec<PageTree>,
    pages: Vec<Page>,
    pub object_number: ObjectNumber,
}
impl PageTree {
    fn new(object_number: ObjectNumber) -> Self {
        let dict = PdfDictionaryObject::new()
            .typed("Pages")
            .with_object_number(object_number);

        PageTree {
            dictionary: dict,
            trees: vec![],
            pages: vec![],
            object_number,
        }
    }

    pub fn with_default_page_size(mut self, page_size: PageSize) -> Self {
        self.dictionary
            .update_or_add("MediaBox", page_size.as_pdf_array());

        self
    }

    pub fn add_tree(&mut self, mut tree: PageTree) -> Result<(), PdfError> {
        tree.dictionary.add("Parent", self.object_number);

        //self.add_kid(Box::new(tree.dictionary))?;
        self.trees.push(tree);

        Ok(())
    }

    pub fn add_page(&mut self, mut page: Page) {
        page.set_parent(self.object_number);
        self.pages.push(page);
    }

    pub fn add_resources() {}

    fn update_counts(&mut self, descendant_page_count: &mut usize) {
        for tree in &mut self.trees {
            tree.update_counts(descendant_page_count)
        }

        *descendant_page_count += self.pages.len();

        self.dictionary.add("Count", *descendant_page_count);
    }

    fn serialize(
        &mut self,
        version: Version,
        xref: &mut XRefOps,
        file: &mut File,
    ) -> Result<(), PdfError> {
        let mut arr = PdfArrayObject::new();
        for page in &mut self.pages {
            arr.push(page.dictionary.object_number.unwrap());
        }
        for tree in &mut self.trees {
            arr.push(tree.object_number);
        }
        self.dictionary.add("Kids", arr);

        self.dictionary.serialize(version, xref, file)?;
        for page in &mut self.pages {
            page.serialize(version, xref, file)?;
        }
        for tree in &mut self.trees{
            tree.serialize(version, xref, file)?;
        }

        Ok(())
    }
}

//--------------------------- ContentsType -------------------------

pub enum ContentsType {
    None,
    Stream,
    Array,
}

//--------------------------- Page ---------------------------------

pub struct Page {
    dictionary: PdfDictionaryObject,
    stream: PdfStreamObject,
    _content_type: ContentsType,
}
impl Page {
    fn new(object_ops: Rc<RefCell<ObjectOps>>, content: Vec<u8>) -> Self {
        let object_number = object_ops.borrow_mut().next_object_number();
        let stream_object_number = object_ops.borrow_mut().next_object_number();
        let mut dict = PdfDictionaryObject::new()
            .typed("Page")
            .with_object_number(object_number);

        // Contents is content stream, or array of streams
        dict.add("Contents", stream_object_number);

        Page {
            dictionary: dict,
            stream:PdfStreamObject::new(stream_object_number).with_content(content),
            _content_type: ContentsType::Stream,
        }
    }

    pub fn with_page_size(mut self, page_size: PageSize) -> Self {
        self.dictionary
            .update_or_add("MediaBox", page_size.as_pdf_array());
        self
    }

    fn set_parent(&mut self, parent_object_number: ObjectNumber) {
        self.dictionary.add("Parent", parent_object_number);
    }

    pub fn object_number(&self)->ObjectNumber{
        self.dictionary.object_number.unwrap()
    }

    pub fn serialize(
        &mut self,
        version: Version,
        xref: &mut XRefOps,
        file: &mut File,
    ) -> Result<(), PdfError> {
        self.dictionary.serialize(version, xref, file)?;
        self.stream.serialize(version, xref, file)?;

        Ok(())
    }
}
