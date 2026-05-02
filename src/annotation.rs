//! Annotation framework for interactive PDF features.
//!
//! Annotations are interactive elements that can be added to PDF pages, including
//! text notes, links, highlights, and form widgets.

use crate::annotation_support::{AdditionalActions, AppearanceCharacteristics, CaptionPosition, FreeTextIntent, HighlightingMode, Poly, Quadding, Shape, TextMarkupType};
use crate::color::ColorsInSpace;
use crate::date::Date;
use crate::util::{Line, Rectangle};
use crate::AnnotationFlags;
use crate::{
    Intent, PdfArrayObject, PdfDictionaryObject, PdfError, PdfNameObject, PdfReferenceObject,
    PdfStreamObject,
};
use crate::object_ops::PdfObj;

pub fn make_annotation_dict(
    subtype: &str,
    rect: Rectangle,
) -> Result<PdfDictionaryObject, PdfError> {
    let mut dict = PdfDictionaryObject::new().typed("Annot")?;
    dict.add("Subtype", subtype)?;
    dict.add("Rect", rect.as_pdf_array_object())?;
    dict.add(
        "Border",
        PdfArrayObject::from_vec_u32(Vec::from([0u32, 0u32, 1u32])),
    )?;
    Ok(dict)
}

macro_rules! impl_annotation {
    ($($t:ty),*) => {
        $(
            impl Annotation for $t {
                fn dict(&mut self) -> &mut PdfDictionaryObject {
                    &mut self.dict
                }
            }
        )*
    };
}

impl_annotation!(
    TextAnnotation,
    LinkAnnotation,
    FreeTextAnnotation,
    LineAnnotation,
    ShapeAnnotation,
    PolyAnnotation,
    TextMarkupAnnotation,
    StampAnnotation,
    CaretAnnotation,
    InkAnnotation,
    PopUpAnnotation,
    FileAttachmentAnnotation,
    SoundAnnotation,
    MovieAnnotation,
    WidgetAnnotation,
    ScrenAnnotation,
    PrintersMarkAnnotation,
    TrapNetworkAnnotation,
    WatermarkAnnotation,
    ThreeDAnnotation,
    RedactAnnotation
);

//------------------- Annotation ----------------------//

pub trait Annotation: Sized {
    fn dict(&mut self) -> &mut PdfDictionaryObject;

    fn with_contents(mut self, contents: &str) -> Result<Self, PdfError> {
        self.dict().add("Contents", contents)?;
        Ok(self)
    }

    fn with_page_indirect_ref(mut self, page_dict: PdfDictionaryObject) -> Result<Self, PdfError> {
        self.dict().add("Name", page_dict)?;
        Ok(self)
    }

    fn with_name(mut self, name: &str) -> Result<Self, PdfError> {
        self.dict().add("NM", PdfObj::name_obj(name))?;
        Ok(self)
    }

    fn with_date_time(mut self, date_time: &str) -> Result<Self, PdfError> {
        self.dict().add("StateModel", date_time)?;
        Ok(self)
    }

    fn with_flags(mut self, flags: AnnotationFlags) -> Result<Self, PdfError> {
        self.dict().add("F", flags.bits())?;
        Ok(self)
    }

    fn with_appearance_dict(mut self, app_dict: PdfDictionaryObject) -> Result<Self, PdfError> {
        self.dict().add("AP", app_dict)?;
        Ok(self)
    }

    fn with_appearance_state(mut self, app_state: &str) -> Result<Self, PdfError> {
        self.dict().add("AS", PdfObj::name_obj(app_state))?;
        Ok(self)
    }

    fn with_border(mut self, border: Vec<u32>) -> Result<Self, PdfError> {
        self.dict().add("Border", border)?;
        Ok(self)
    }

    fn with_color(mut self, color: PdfArrayObject) -> Result<Self, PdfError> {
        self.dict().add("C", color)?;
        Ok(self)
    }

    fn with_struct_parent(mut self, parent_num: u64) -> Result<Self, PdfError> {
        self.dict().add("StructParent", parent_num)?;
        Ok(self)
    }

    fn with_content_group(mut self, dict: PdfDictionaryObject) -> Result<Self, PdfError> {
        self.dict().add("OC", dict)?;
        Ok(self)
    }
}

//------------------- TextAnnotation ----------------------//

pub struct TextAnnotation {
    dict: PdfDictionaryObject,
}
impl TextAnnotation {
    pub fn new(rect: Rectangle) -> Result<Self, PdfError> {
        Ok(Self {
            dict: make_annotation_dict("Text", rect)?,
        })
    }

    pub fn with_open(mut self, open: bool) -> Result<Self, PdfError> {
        self.dict.add("Open", open)?;
        Ok(self)
    }

    pub fn with_name(mut self, name: &str) -> Result<Self, PdfError> {
        self.dict.add("Name", PdfObj::name_obj(name))?;
        Ok(self)
    }

    pub fn with_state(mut self, state: &str) -> Result<Self, PdfError> {
        self.dict.add("State", state)?;
        Ok(self)
    }

    pub fn with_state_model(mut self, state_model: &str) -> Result<Self, PdfError> {
        self.dict
            .add("StateModel", state_model)?;
        Ok(self)
    }
}

//-------------------LinkAnnotation ----------------------

pub struct LinkAnnotation {
    dict: PdfDictionaryObject,
}
impl LinkAnnotation {
    pub fn new(rect: Rectangle) -> Result<Self, PdfError> {
        Ok(Self {
            dict: make_annotation_dict("Link", rect)?,
        })
    }

    pub fn with_action(mut self, action_dict: PdfDictionaryObject) -> Result<Self, PdfError> {
        self.dict.add("A", action_dict)?;
        Ok(self)
    }

    pub fn with_dest_array(mut self, dest_arr: PdfArrayObject) -> Result<Self, PdfError> {
        self.dict.add("Dest", dest_arr)?;
        Ok(self)
    }

    pub fn with_dest_string(mut self, dest: &str) -> Result<Self, PdfError> {
        self.dict.add("Dest", dest)?;
        Ok(self)
    }

    pub fn with_dest_name(mut self, name: PdfNameObject) -> Result<Self, PdfError> {
        self.dict.add("Dest", name)?;
        Ok(self)
    }

    pub fn with_highlighting_mode(
        mut self,
        mode_dict: PdfDictionaryObject,
    ) -> Result<Self, PdfError> {
        self.dict.add("H", mode_dict)?;
        Ok(self)
    }

    pub fn with_uri_action(
        mut self,
        uri_action_dict: PdfDictionaryObject,
    ) -> Result<Self, PdfError> {
        self.dict.add("PA", uri_action_dict)?;
        Ok(self)
    }

    pub fn with_quad_points(mut self, quad_points_arr: PdfArrayObject) -> Result<Self, PdfError> {
        self.dict.add("QuadPoints", quad_points_arr)?;
        Ok(self)
    }

    pub fn with_border_style(mut self, style_dict: PdfDictionaryObject) -> Result<Self, PdfError> {
        self.dict.add("BS", style_dict)?;
        Ok(self)
    }
}

//-------------------FreeTextAnnotation ----------------------//

pub struct FreeTextAnnotation {
    dict: PdfDictionaryObject,
}
impl FreeTextAnnotation {
    pub fn new(rect: Rectangle, default_appearance: &str) -> Result<Self, PdfError> {
        let mut ann_dict = make_annotation_dict("FreeText", rect)?;
        ann_dict.add("DA", default_appearance)?;
        ann_dict.add("IT", FreeTextIntent::FreeText as u8)?; // default
        ann_dict.add("LE", PdfObj::name_obj("None"))?; // default

        Ok(Self { dict: ann_dict })
    }

    pub fn with_quadding(mut self, quadding: Quadding) -> Result<Self, PdfError> {
        self.dict.add("Q", quadding as u8)?;
        Ok(self)
    }

    pub fn with_rich_text_string(mut self, rich_text_string: &str) -> Result<Self, PdfError> {
        self.dict.add("RC", rich_text_string)?;
        Ok(self)
    }

    pub fn with_rich_text_stream(
        mut self,
        rich_text_stream: PdfStreamObject,
    ) -> Result<Self, PdfError> {
        self.dict.add("RC", rich_text_stream)?;
        Ok(self)
    }

    pub fn with_style_string(mut self, styler_str: &str) -> Result<Self, PdfError> {
        self.dict.add("DS", styler_str)?;
        Ok(self)
    }

    pub fn with_callout_line_string(
        mut self,
        callout_line_arr: PdfArrayObject,
    ) -> Result<Self, PdfError> {
        self.dict.add("CL", callout_line_arr)?;
        Ok(self)
    }

    pub fn with_intent(mut self, intent: FreeTextIntent) -> Result<Self, PdfError> {
        self.dict.update_or_add("IT", intent as u8);
        Ok(self)
    }

    pub fn with_border_effects(
        mut self,
        border_effects_dict: PdfDictionaryObject,
    ) -> Result<Self, PdfError> {
        self.dict.add("BE", border_effects_dict)?;
        Ok(self)
    }

    pub fn with_rectangle_diffs(mut self, rect_diffs_arr: Rectangle) -> Result<Self, PdfError> {
        self.dict.add("RD", rect_diffs_arr.as_pdf_array_object())?;
        Ok(self)
    }

    pub fn with_border_style(mut self, style_dict: PdfDictionaryObject) -> Result<Self, PdfError> {
        self.dict.add("BS", style_dict)?;
        Ok(self)
    }

    pub fn with_line_ending_style(mut self, style: &str) -> Result<Self, PdfError> {
        self.dict.add("LE", PdfObj::name_obj(style))?;
        Ok(self)
    }
}

//-------------------LineAnnotation ----------------------//

pub struct LineAnnotation {
    dict: PdfDictionaryObject,
}
impl LineAnnotation {
    pub fn new(rect: Rectangle, line: Line) -> Result<Self, PdfError> {
        let mut dict = make_annotation_dict("Line", rect)?;
        dict.add("L", line.as_pdf_array_object())?;
        let mut arr = PdfArrayObject::new();
        arr.push(PdfObj::name_obj("None"));
        arr.push(PdfObj::name_obj("None"));
        // defaults
        dict.add("LE", arr)?;
        dict.add("LL", PdfObj::num_obj(0))?;
        dict.add("LLE", PdfObj::num_obj(0))?;
        dict.add("Cap", false)?;
        dict.add("CP", PdfObj::name_obj("Inline"))?;
        dict.add("CO", PdfArrayObject::from_vec_f64(vec![0.0, 0.0]))?;

        Ok(Self {
            dict: make_annotation_dict("Line", rect)?,
        })
    }

    pub fn with_border_style(mut self, style_dict: PdfDictionaryObject) -> Result<Self, PdfError> {
        self.dict.add("BS", style_dict)?;
        Ok(self)
    }

    pub fn with_line_ending_style(mut self, first: &str, second: &str) -> Result<Self, PdfError> {
        let mut arr = PdfArrayObject::new();
        arr.push(PdfObj::name_obj(first));
        arr.push(PdfObj::name_obj(second));
        self.dict.add("LE", arr)?;
        Ok(self)
    }

    pub fn with_interior_colors(mut self, colors: ColorsInSpace) -> Result<Self, PdfError> {
        self.dict.add("IC", colors.as_pdf_array())?;
        Ok(self)
    }

    pub fn with_leader_lines_length(mut self, length: f64) -> Result<Self, PdfError> {
        self.dict.add("LL", PdfObj::num_obj(length))?;
        Ok(self)
    }

    pub fn with_leader_lines_extension_length(mut self, length: f64) -> Result<Self, PdfError> {
        self.dict.add("LLE", PdfObj::num_obj(length))?;
        Ok(self)
    }

    pub fn with_caption(mut self, make_caption: bool) -> Result<Self, PdfError> {
        self.dict.add("Cap", make_caption)?;
        Ok(self)
    }

    pub fn with_intent(mut self, intent: Intent) -> Result<Self, PdfError> {
        self.dict
            .add("IT", PdfObj::name_obj(&*intent.to_string()))?;
        Ok(self)
    }

    pub fn with_leader_line_offset_length(mut self, length: u64) -> Result<Self, PdfError> {
        self.dict.add("LLO", PdfObj::num_obj(length))?;
        Ok(self)
    }

    pub fn with_caption_position(mut self, position: CaptionPosition) -> Result<Self, PdfError> {
        self.dict
            .add("CP", PdfObj::name_obj(&*position.to_string()))?;
        Ok(self)
    }

    pub fn with_measure_dict(
        mut self,
        measure_dict: PdfDictionaryObject,
    ) -> Result<Self, PdfError> {
        self.dict.add("Measure", measure_dict)?;
        Ok(self)
    }

    pub fn with_caption_offsets(
        mut self,
        horizontal: f64,
        vertical: f64,
    ) -> Result<Self, PdfError> {
        self.dict.add(
            "CO",
            PdfArrayObject::from_vec_f64(Vec::from([horizontal, vertical])),
        )?;
        Ok(self)
    }
}

//-------------------ShapeAnnotation ----------------------//

pub struct ShapeAnnotation {
    dict: PdfDictionaryObject,
}
impl ShapeAnnotation {
    pub fn new(rect: Rectangle, shape: Shape) -> Result<Self, PdfError> {
        Ok(Self {
            dict: make_annotation_dict(&*shape.to_string(), rect)?,
        })
    }

    pub fn with_border_style(mut self, style_dict: PdfDictionaryObject) -> Result<Self, PdfError> {
        self.dict.add("BS", style_dict)?;
        Ok(self)
    }

    pub fn with_interior_style(mut self, colors: ColorsInSpace) -> Result<Self, PdfError> {
        self.dict.add("IC", colors.as_pdf_array())?;
        Ok(self)
    }

    pub fn with_border_effects(
        mut self,
        border_effects_dict: PdfDictionaryObject,
    ) -> Result<Self, PdfError> {
        self.dict.add("BE", border_effects_dict)?;
        Ok(self)
    }

    pub fn with_boundary_offsets(
        mut self,
        left: f64,
        top: f64,
        right: f64,
        bottom: f64,
    ) -> Result<Self, PdfError> {
        if left < 0.0 || top < 0.0 || right < 0.0 || bottom < 0.0 {
            return Err(PdfError::InvalidArgument(
                "Boundary offsets cannot be negative".to_string(),
            ));
        }
        let rect_arr = self.dict.get("Rect").unwrap().as_vec_f64()?;
        let rleft = rect_arr[0];
        let rtop = rect_arr[1];
        let rright = rect_arr[2];
        let rbottom = rect_arr[3];
        if top + bottom >= rtop - rbottom || left + right >= rleft - rright {
            return Err(PdfError::InvalidArgument(
                "Boundary offsets cannot be outside of the rectangle".to_string(),
            ));
        }
        self.dict.add(
            "RD",
            PdfArrayObject::from_vec_f64(Vec::from([left, top, right, bottom])),
        )?;
        Ok(self)
    }
}

//-------------------PolyAnnotation ----------------------//

pub struct PolyAnnotation {
    dict: PdfDictionaryObject,
}

impl PolyAnnotation {
    pub fn new(rect: Rectangle, poly: Poly) -> Result<Self, PdfError> {
        Ok(Self {
            dict: make_annotation_dict(&*poly.to_string(), rect)?,
        })
    }
}

//-------------------PolylineAnnotation ----------------------//

pub struct PolyLineAnnotation {
    dict: PdfDictionaryObject,
}

impl PolyLineAnnotation {
    pub fn new(rect: Rectangle, vertices: PdfArrayObject) -> Result<Self, PdfError> {
        let mut dict = make_annotation_dict("PolyLine", rect)?;
        dict.add("Vertices", vertices)?;
        let mut arr = PdfArrayObject::new();
        arr.push(PdfObj::name_obj("None"));
        arr.push(PdfObj::name_obj("None"));
        dict.add("LE", arr)?;
        Ok(Self { dict })
    }

    pub fn with_line_ending_styles(mut self, first: &str, second: &str) -> Result<Self, PdfError> {
        if !self.dict.contains_key("PolyLine") {
            return Err(PdfError::InvalidArgument(
                "Cannot set line ending styles for a polygon annotation".to_string(),
            ));
        }
        let mut arr = PdfArrayObject::new();
        arr.push(PdfObj::name_obj(first));
        arr.push(PdfObj::name_obj(second));
        self.dict.add("LE", arr)?;
        Ok(self)
    }

    pub fn with_border_styles(mut self, style_dict: PdfDictionaryObject) -> Result<Self, PdfError> {
        self.dict.add("BS", style_dict)?;
        Ok(self)
    }

    pub fn with_interior_colors(mut self, colors: ColorsInSpace) -> Result<Self, PdfError> {
        self.dict.add("IC", colors.as_pdf_array())?;
        Ok(self)
    }

    pub fn with_border_effects(
        mut self,
        border_effects_dict: PdfDictionaryObject,
    ) -> Result<Self, PdfError> {
        if !self.dict.contains_key("Polygon") {
            return Err(PdfError::InvalidArgument(
                "Cannot set border effects for a polyline annotation".to_string(),
            ));
        }
        self.dict.add("BE", border_effects_dict)?;
        Ok(self)
    }

    pub fn with_intent(mut self, intent: Intent) -> Result<Self, PdfError> {
        self.dict
            .add("IT", PdfObj::name_obj(&*intent.to_string()))?;
        Ok(self)
    }

    pub fn with_measure(mut self, measure_dict: PdfDictionaryObject) -> Result<Self, PdfError> {
        self.dict.add("Measure", measure_dict)?;
        Ok(self)
    }
}

//-------------------TextMarkupAnnotation ----------------------//

pub struct TextMarkupAnnotation {
    dict: PdfDictionaryObject,
}
impl TextMarkupAnnotation {
    pub fn new(rect: Rectangle, text_markup: TextMarkupType) -> Result<Self, PdfError> {
        Ok(Self {
            dict: make_annotation_dict(&*text_markup.to_string(), rect)?,
        })
    }

    pub fn with_quad_points(mut self, quad_points: PdfArrayObject) -> Result<Self, PdfError> {
        self.dict.add("QuadPoints", quad_points)?;
        Ok(self)
    }

    pub fn with_text_string(mut self, text_string: &str) -> Result<Self, PdfError> {
        self.dict.add("Contents", text_string)?;
        Ok(self)
    }
}

//-------------------CaretAnnotation ----------------------//

pub struct CaretAnnotation {
    dict: PdfDictionaryObject,
}
impl CaretAnnotation {
    pub fn new(rect: Rectangle) -> Result<Self, PdfError> {
        let mut dict = make_annotation_dict("Caret", rect)?;
        dict.add("Sy", PdfObj::name_obj("None"))?;
        Ok(Self { dict })
    }

    pub fn with_rec_offsets(
        mut self,
        left: f64,
        top: f64,
        right: f64,
        bottom: f64,
    ) -> Result<Self, PdfError> {
        if left < 0.0 || top < 0.0 || right < 0.0 || bottom < 0.0 {
            return Err(PdfError::InvalidArgument(
                "Rec offsets cannot be negative".to_string(),
            ));
        }
        let rect_arr = self.dict.get("Rect").unwrap().as_vec_f64()?;
        let rleft = rect_arr[0];
        let rtop = rect_arr[1];
        let rright = rect_arr[2];
        let rbottom = rect_arr[3];
        if left > rright || right < rleft || top > rtop || bottom < rbottom {
            return Err(PdfError::InvalidArgument(
                "Rec offsets are outside of annotation rectangle".to_string(),
            ));
        }
        self.dict.add(
            "RD",
            PdfArrayObject::from_vec_f64(Vec::from([left, top, right, bottom])),
        )?;

        Ok(self)
    }

    pub fn with_symbol(mut self) -> Result<Self, PdfError> {
        self.dict.add("Symbol", PdfObj::name_obj("P"))?;
        Ok(self)
    }
}

//-------------------StampAnnotation ----------------------//

pub struct StampAnnotation {
    dict: PdfDictionaryObject,
}
impl StampAnnotation {
    pub fn new(rect: Rectangle) -> Result<Self, PdfError> {
        let mut dict = make_annotation_dict("Stamp", rect)?;
        dict.add("Name", PdfObj::name_obj("Draft"))?;
        Ok(Self { dict })
    }

    pub fn with_name(mut self, name: &str) -> Result<Self, PdfError> {
        self.dict.add("Name", PdfObj::name_obj(name))?;
        Ok(self)
    }
}

//-------------------InkAnnotation ----------------------//

pub struct InkAnnotation {
    dict: PdfDictionaryObject,
}
impl InkAnnotation {
    pub fn new(rect: Rectangle) -> Result<Self, PdfError> {
        Ok(Self {
            dict: make_annotation_dict("Ink", rect)?,
        })
    }

    pub fn with_ink_list(mut self, ink_list: PdfArrayObject) -> Result<Self, PdfError> {
        self.dict.add("InkList", ink_list)?;
        Ok(self)
    }

    pub fn with_border_style(mut self, style_dict: PdfDictionaryObject) -> Result<Self, PdfError> {
        self.dict.add("BS", style_dict)?;
        Ok(self)
    }
}

//-------------------PopupAnnotation ----------------------//

pub struct PopUpAnnotation {
    dict: PdfDictionaryObject,
}
impl PopUpAnnotation {
    pub fn new(rect: Rectangle) -> Result<Self, PdfError> {
        let mut dict = make_annotation_dict("Popup", rect)?;
        dict.add("Open", false)?;
        Ok(Self { dict })
    }

    pub fn with_parent(mut self, parent: PdfReferenceObject) -> Result<Self, PdfError> {
        self.dict.add("Parent", parent)?;
        Ok(self)
    }

    pub fn with_open(mut self, open: bool) -> Result<Self, PdfError> {
        self.dict.add("Open", open)?;
        Ok(self)
    }
}

//-------------------FileAttachmentAnnotation ----------------------//

pub struct FileAttachmentAnnotation {
    dict: PdfDictionaryObject,
}
impl FileAttachmentAnnotation {
    pub fn new(rect: Rectangle, file_spec: &str) -> Result<Self, PdfError> {
        let mut dict = make_annotation_dict("FileAttachment", rect)?;
        dict.add("FS", PdfObj::name_obj(file_spec))?;
        Ok(Self { dict })
    }

    pub fn with_name(mut self, name: &str) -> Result<Self, PdfError> {
        self.dict.add("Name", PdfObj::name_obj(name))?;
        Ok(self)
    }
}

//-------------------SoundAnnotation ----------------------//

pub struct SoundAnnotation {
    dict: PdfDictionaryObject,
}
impl SoundAnnotation {
    pub fn new(rect: Rectangle, sound_stream: PdfStreamObject) -> Result<Self, PdfError> {
        let mut dict = make_annotation_dict("Sound", rect)?;
        dict.add("Sound", sound_stream)?;
        Ok(Self { dict })
    }

    pub fn with_name(mut self, name: &str) -> Result<Self, PdfError> {
        self.dict.add("Name", PdfObj::name_obj(name))?;
        Ok(self)
    }
}

//-------------------MovieAnnotation ----------------------//

pub struct MovieAnnotation {
    dict: PdfDictionaryObject,
}
impl MovieAnnotation {
    pub fn new(rect: Rectangle, movie_dict: PdfDictionaryObject) -> Result<Self, PdfError> {
        let mut dict = make_annotation_dict("Movie", rect)?;
        dict.add("Movie", movie_dict)?;
        dict.add("A", true)?;
        Ok(Self { dict })
    }

    pub fn with_title(mut self, title: &str) -> Result<Self, PdfError> {
        self.dict.add("T", title)?;
        Ok(self)
    }

    pub fn with_play_method_bool(mut self, play_method_bool: bool) -> Result<Self, PdfError> {
        self.dict.add("A", play_method_bool)?;
        Ok(self)
    }

    pub fn with_play_method_dict(
        mut self,
        play_method_dict: PdfDictionaryObject,
    ) -> Result<Self, PdfError> {
        self.dict.add("A", play_method_dict)?;
        Ok(self)
    }
}

//-------------------ScreenAnnotation ----------------------//

pub struct ScrenAnnotation {
    dict: PdfDictionaryObject,
}
impl ScrenAnnotation {
    pub fn new(rect: Rectangle) -> Result<Self, PdfError> {
        Ok(Self {
            dict: make_annotation_dict("Screen", rect)?,
        })
    }

    pub fn with_title(mut self, title: &str) -> Result<Self, PdfError> {
        self.dict.add("T", title)?;
        Ok(self)
    }

    pub fn with_appearance_characteristics(
        mut self,
        appearance_characteristics: AppearanceCharacteristics,
    ) -> Result<Self, PdfError> {
        self.dict.add("MK", appearance_characteristics.dict)?;
        Ok(self)
    }

    pub fn with_action(mut self, action: PdfDictionaryObject) -> Result<Self, PdfError> {
        self.dict.add("A", action)?;
        Ok(self)
    }

    pub fn with_additional_actions(
        mut self,
        additional_actions: AdditionalActions,
    ) -> Result<Self, PdfError> {
        self.dict.add("AA", additional_actions.dict)?;
        Ok(self)
    }
}

//-------------------WidgetAnnotation ----------------------//

pub struct WidgetAnnotation {
    dict: PdfDictionaryObject,
}
impl WidgetAnnotation {
    pub fn new(rect: Rectangle) -> Result<Self, PdfError> {
        Ok(Self {
            dict: make_annotation_dict("Widget", rect)?,
        })
    }

    pub fn with_highlighting(
        mut self,
        highlighting_mode: HighlightingMode,
    ) -> Result<Self, PdfError> {
        self.dict.add("H", highlighting_mode.to_pdf_string())?;
        Ok(self)
    }

    pub fn with_appearance_characteristics(
        mut self,
        appearance_characteristics: AppearanceCharacteristics,
    ) -> Result<Self, PdfError> {
        self.dict.add("MK", appearance_characteristics.dict)?;
        Ok(self)
    }

    pub fn with_action(mut self, action: PdfDictionaryObject) -> Result<Self, PdfError> {
        self.dict.add("A", action)?;
        Ok(self)
    }

    pub fn with_additional_actions(
        mut self,
        additional_actions: AdditionalActions,
    ) -> Result<Self, PdfError> {
        self.dict.add("AA", additional_actions.dict)?;
        Ok(self)
    }

    pub fn with_border_style(mut self, style_dict: PdfDictionaryObject) -> Result<Self, PdfError> {
        self.dict.add("BS", style_dict)?;
        Ok(self)
    }

    pub fn with_parent(mut self, parent: PdfReferenceObject) -> Result<Self, PdfError> {
        self.dict.add("Parent", parent)?;
        Ok(self)
    }
}

//-------------------PrinterMarkAnnotation ----------------------//

pub struct PrintersMarkAnnotation {
    dict: PdfDictionaryObject,
}
impl PrintersMarkAnnotation {
    pub fn new(rect: Rectangle) -> Result<Self, PdfError> {
        Ok(Self {
            dict: make_annotation_dict("PrinterMark", rect)?,
        })
    }

    pub fn with_type(mut self, type_: &str) -> Result<Self, PdfError> {
        self.dict.add("MN", PdfObj::name_obj(type_))?;
        Ok(self)
    }
}

//-------------------TrapNetworkAnnotation ----------------------//

pub struct TrapNetworkAnnotation {
    dict: PdfDictionaryObject,
}
impl TrapNetworkAnnotation {
    pub fn new(rect: Rectangle) -> Result<Self, PdfError> {
        Ok(Self {
            dict: make_annotation_dict("TrapNet", rect)?,
        })
    }

    pub fn with_last_modified(mut self, last_modified: Date)  -> Result<Self, PdfError> {
        self.dict.add("LastModified", last_modified.to_pdf_string())?;
        Ok(self)
    }
}

//-------------------WatermarkAnnotation ----------------------//

pub struct WatermarkAnnotation {
    dict: PdfDictionaryObject,
}
impl WatermarkAnnotation {
    pub fn new(rect: Rectangle) -> Result<Self, PdfError> {
        Ok(Self {
            dict: make_annotation_dict("Watermark", rect)?,
        })
    }

    pub fn with_fixed_print(mut self, fixed_print: PdfDictionaryObject) -> Result<Self, PdfError> {
        self.dict.add("FixedPrint", fixed_print)?;
        Ok(self)
    }
}

//-------------------3DAnnotation ----------------------//

pub struct ThreeDAnnotation {
    dict: PdfDictionaryObject,
}
impl ThreeDAnnotation {
    pub fn new(rect: Rectangle) -> Result<Self, PdfError> {
        Ok(Self {
            dict: make_annotation_dict("3D", rect)?,
        })
    }
}

//-------------------RedactAnnotation ----------------------//

pub struct RedactAnnotation {
    dict: PdfDictionaryObject,
}
impl RedactAnnotation {
    pub fn new(rect: Rectangle) -> Result<Self, PdfError> {
        Ok(Self {
            dict: make_annotation_dict("Redact", rect)?,
        })
    }
}
