use crate::color::{ColorsInSpace, RGB};
use crate::util::Posn;
use crate::{
    PdfArrayObject, PdfDictionaryObject, PdfError, PdfReferenceObject, PdfStreamObject,
    PdfStringObject,
};
//-------------------AnnotationFlags ----------------------//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnnotationFlags(u32);

impl AnnotationFlags {
    pub const NONE: Self = Self(0);
    pub const INVISIBLE: Self = Self(1 << 0);
    pub const HIDDEN: Self = Self(1 << 1);
    pub const PRINT: Self = Self(1 << 2);
    pub const NO_ZOOM: Self = Self(1 << 3);
    pub const NO_ROTATE: Self = Self(1 << 4);
    pub const NO_VIEW: Self = Self(1 << 5);
    pub const READ_ONLY: Self = Self(1 << 6);
    pub const LOCKED: Self = Self(1 << 7);

    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    pub const fn bits(&self) -> u32 {
        self.0
    }

    pub fn is_empty(&self) -> bool {
        self.bits() == 0
    }

    pub const fn or(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

//-------------------Intent ----------------------//

pub enum Intent {
    LineArrow,
    LineDimension,
}
impl Intent {
    pub fn to_string(&self) -> String {
        match self {
            Intent::LineArrow => "LineArrow".to_string(),
            Intent::LineDimension => "LineDimension".to_string(),
        }
    }
}

//-------------------AppearanceCharacteristics ----------------------//

pub struct AppearanceCharacteristics {
    pub dict: PdfDictionaryObject,
}
impl AppearanceCharacteristics {
    pub fn new() -> Result<Self, PdfError> {
        let mut dict = PdfDictionaryObject::new();
        dict.add("R", 0)?;
        Ok(Self { dict })
    }

    pub fn with_rotation(&mut self, rotation: i32) -> Result<&mut Self, PdfError> {
        self.dict.add("R", rotation)?;
        Ok(self)
    }

    pub fn with_border_color(&mut self, color: ColorsInSpace) -> Result<&mut Self, PdfError> {
        self.dict.add("BC", color.as_pdf_array())?;
        Ok(self)
    }

    pub fn with_background_color(&mut self, color: ColorsInSpace) -> Result<&mut Self, PdfError> {
        self.dict.add("BG", color.as_pdf_array())?;
        Ok(self)
    }

    pub fn with_caption(&mut self, caption: &str) -> Result<&mut Self, PdfError> {
        self.dict.add("C", caption)?;
        Ok(self)
    }

    pub fn with_rollover_caption(&mut self, caption: &str) -> Result<&mut Self, PdfError> {
        self.dict.add("RC", caption)?;
        Ok(self)
    }

    pub fn with_alternate_caption(&mut self, caption: &str) -> Result<&mut Self, PdfError> {
        self.dict.add("AC", caption)?;
        Ok(self)
    }

    pub fn with_normal_icon(&mut self, icon: PdfReferenceObject) -> Result<&mut Self, PdfError> {
        self.dict.add("I", icon)?;
        Ok(self)
    }

    pub fn with_rollover_icon(&mut self, icon: PdfReferenceObject) -> Result<&mut Self, PdfError> {
        self.dict.add("RI", icon)?;
        Ok(self)
    }

    pub fn with_alternate_icon(&mut self, icon: PdfReferenceObject) -> Result<&mut Self, PdfError> {
        self.dict.add("IX", icon)?;
        Ok(self)
    }

    pub fn with_icon_fir(mut self, icon_fir: PdfReferenceObject) -> Result<Self, PdfError> {
        self.dict.add("IF", icon_fir)?;
        Ok(self)
    }

    pub fn with_caption_posn(&mut self, posn: CaptionPosition) -> Result<&mut Self, PdfError> {
        self.dict.add("CP", posn.to_string())?;
        Ok(self)
    }
}

//-------------------AdditionalActions ----------------------//

pub struct AdditionalActions {
    pub dict: PdfDictionaryObject,
}
impl AdditionalActions {
    pub fn new() -> Result<Self, PdfError> {
        Ok(Self {
            dict: PdfDictionaryObject::new(),
        })
    }
}

//-------------------CaptionPosition ----------------------//

pub enum CaptionPosition {
    NoIcon,
    NoCaption,
    Below,
    Above,
    Left,
    Right,
    Center,
}
impl CaptionPosition {
    pub fn to_string(&self) -> String {
        match self {
            CaptionPosition::NoIcon => "0".to_string(),
            CaptionPosition::NoCaption => "1".to_string(),
            CaptionPosition::Below => "2".to_string(),
            CaptionPosition::Above => "3".to_string(),
            CaptionPosition::Left => "4".to_string(),
            CaptionPosition::Right => "5".to_string(),
            CaptionPosition::Center => "6".to_string(),
        }
    }
}

//-------------------TextMarkupType ----------------------//

pub enum TextMarkupType {
    Highlight,
    Underline,
    Squiggly,
    StrikeOut,
}

impl TextMarkupType {
    pub fn to_string(&self) -> String {
        match self {
            TextMarkupType::Highlight => "Highlight".to_string(),
            TextMarkupType::Underline => "Underline".to_string(),
            TextMarkupType::Squiggly => "Squiggly".to_string(),
            TextMarkupType::StrikeOut => "StrikeOut".to_string(),
        }
    }
}

pub enum ActionTypes {}

//-------------------BorderStyle ----------------------//

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BorderStyle {
    Solid,
    Dashed,
    Beveled,
    Inset,
    Underline,
}

impl BorderStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            BorderStyle::Solid => "S",
            BorderStyle::Dashed => "D",
            BorderStyle::Beveled => "B",
            BorderStyle::Inset => "I",
            BorderStyle::Underline => "U",
        }
    }
}

//------------------- TextIcon ----------------------//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextIcon {
    Comment,
    Key,
    Note,
    Help,
    NewParagraph,
    Paragraph,
    Insert,
}

impl TextIcon {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextIcon::Comment => "Comment",
            TextIcon::Key => "Key",
            TextIcon::Note => "Note",
            TextIcon::Help => "Help",
            TextIcon::NewParagraph => "NewParagraph",
            TextIcon::Paragraph => "Paragraph",
            TextIcon::Insert => "Insert",
        }
    }
}

//-------------------LinkAction ----------------------

#[derive(Debug, Clone)]
pub enum LinkAction {
    Uri(String),
    GoTo {
        page: usize,
        position: Posn,
        zoom: Option<f64>,
    },
}

pub enum HighlightingMode {
    None,
    Invert,
    Outline,
    Push,
    Toggle,
}
impl HighlightingMode {
    pub(crate) fn to_pdf_string(&self) -> PdfStringObject {
        match self {
            HighlightingMode::None => PdfStringObject::new("N"),
            HighlightingMode::Invert => PdfStringObject::new("I"),
            HighlightingMode::Outline => PdfStringObject::new("O"),
            HighlightingMode::Push => PdfStringObject::new("P"),
            HighlightingMode::Toggle => PdfStringObject::new("T"),
        }
    }
}

pub struct BoxColorInformation {
    pub dict: PdfDictionaryObject,
}
impl BoxColorInformation {
    pub fn new() -> Result<Self, PdfError> {
        Ok(Self {
            dict: PdfDictionaryObject::new(),
        })
    }

    pub fn with_crop_box(&mut self, box_style: BoxStyle) -> Result<&mut Self, PdfError> {
        self.dict.add("C", box_style.dict)?;
        Ok(self)
    }

    pub fn with_bleed_box(&mut self, box_style: BoxStyle) -> Result<&mut Self, PdfError> {
        self.dict.add("B", box_style.dict)?;
        Ok(self)
    }
}

pub struct BoxStyle {
    pub dict: PdfDictionaryObject,
}
impl BoxStyle {
    pub fn new() -> Result<Self, PdfError> {
        let mut dict = PdfDictionaryObject::new();
        dict.add("C", RGB::BLACK.as_pdf_array())?;
        dict.add("W", 1.0)?;
        dict.add("S", GuidelineStyle::Solid.as_string())?;
        dict.add("D", PdfArrayObject::from_vec_f32(vec![3.0]))?;

        Ok(Self { dict })
    }

    pub fn with_color(&mut self, rgb: RGB) -> Result<&mut Self, PdfError> {
        self.dict.add("C", rgb.as_pdf_array())?;
        Ok(self)
    }

    pub fn with_guideline_width(&mut self, width: f32) -> Result<&mut Self, PdfError> {
        self.dict.add("W", width)?;
        Ok(self)
    }

    pub fn with_guideline_style(&mut self, style: GuidelineStyle) -> Result<&mut Self, PdfError> {
        self.dict.add("S", style.as_string())?;
        Ok(self)
    }

    pub fn with_dash_pattern(&mut self, pattern: Vec<f32>) -> Result<&mut Self, PdfError> {
        self.dict.add("D", PdfArrayObject::from_vec_f32(pattern))?;
        Ok(self)
    }
}

pub enum GuidelineStyle {
    Solid,
    Dashed,
}
impl GuidelineStyle {
    pub fn as_string(&self) -> &str {
        match self {
            GuidelineStyle::Solid => "S",
            GuidelineStyle::Dashed => "D",
        }
    }
}

pub struct Separation {
    pub dict: PdfDictionaryObject,
}
impl Separation {
    pub fn new(pages: PdfArrayObject, device_colorant: &str) -> Result<Self, PdfError> {
        let mut dict = PdfDictionaryObject::new();
        dict.add("Pages", pages)?;
        dict.add("DeviceColorant", device_colorant)?;

        Ok(Self { dict })
    }

    pub fn with_color_space(&mut self, color_space: PdfArrayObject) -> Result<&mut Self, PdfError> {
        self.dict.add("ColorSpace", color_space)?;
        Ok(self)
    }
}

pub struct OutputIntent {
    pub dict: PdfDictionaryObject,
}
impl OutputIntent {
    pub fn new(
        subtype: OutputIntentSubtype,
        output_condition_identifier: &str,
    ) -> Result<Self, PdfError> {
        let mut dict = PdfDictionaryObject::new().typed("OutputIntent")?;
        dict.add("S", subtype.as_string())?;
        dict.add("OutputConditionIdentifier", output_condition_identifier)?;
        Ok(Self { dict })
    }

    pub fn with_output_condition(mut self, output_condition: &str) -> Result<Self, PdfError> {
        self.dict.add("OutputCondition", output_condition)?;
        Ok(self)
    }

    pub fn with_registry_name(&mut self, registry_name: &str) -> Result<&mut Self, PdfError> {
        self.dict.add("RegistryName", registry_name)?;
        Ok(self)
    }

    pub fn with_info(&mut self, info: &str) -> Result<&mut Self, PdfError> {
        self.dict.add("Info", info)?;
        Ok(self)
    }

    pub fn withdest_output_profile(
        &mut self,
        dest_output_profile: PdfStreamObject,
    ) -> Result<&mut Self, PdfError> {
        self.dict.add("DestOutputProfile", dest_output_profile)?;
        Ok(self)
    }
}

pub enum OutputIntentSubtype {
    GtsPdfx,
    GtsPdfa1,
    IsoPdfe1,
}
impl OutputIntentSubtype {
    pub fn as_string(&self) -> &str {
        match self {
            OutputIntentSubtype::GtsPdfx => "GTS_PDFX",
            OutputIntentSubtype::GtsPdfa1 => "GTS_PDFA1",
            OutputIntentSubtype::IsoPdfe1 => "ISO_PDFE1",
        }
    }
}

pub enum FreeTextIntent {
    FreeText,
    FreeTextCallout,
    FreeTextTypeWriter,
}

pub enum Quadding {
    Left = 0,
    Center = 1,
    Right = 2,
}

pub enum Poly {
    Polygon,
    PolyLine,
}
impl Poly {
    pub fn to_string(&self) -> String {
        match self {
            Poly::Polygon => "Polygon".to_string(),
            Poly::PolyLine => "PolyLine".to_string(),
        }
    }
}

pub enum Shape {
    Square,
    Circle,
}
impl Shape {
    pub fn to_string(&self) -> String {
        match self {
            Shape::Square => "Square".to_string(),
            Shape::Circle => "Circle".to_string(),
        }
    }
}

