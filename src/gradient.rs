use std::sync::Arc;

use crate::objects::base::IndirectReference;
use crate::objects::stream::StreamObject;
use crate::util::{DimsPoints, PosnPoints, RGBA};
use crate::{
    Array, ArrayObject, BooleanObject, DictionaryObject, NameObject, NumberObject, PDF, PdfObject,
};

//--------------------------- Color Stop ---------------------------//

#[derive(Debug, Clone)]
pub struct ColorStop {
    pub offset: f32, // along the gradient (0.0 = start, 1.0 = end)
    pub rgba: RGBA,
}

impl ColorStop {
    pub fn new(offset: f32, rgba: RGBA) -> Self {
        ColorStop { offset, rgba }
    }
}

//--------------------------- Gradient Kind ---------------------------//

#[derive(Debug, Clone)]
pub enum GradientKind {
    Linear { angle: f32 }, // (CSS convention: 0° is north/up, clockwise).
    Radial,
}

//--------------------------- Gradient ---------------------------//

pub struct Gradient {
    pub stops: Vec<ColorStop>,
    pub kind: GradientKind,
}

impl Gradient {
    pub fn new(kind: GradientKind) -> Self {
        Gradient {
            stops: Vec::new(),
            kind,
        }
    }

    pub fn add_stop(&mut self, offset: f32, rgba: RGBA) {
        self.stops.push(ColorStop::new(offset, rgba));
    }

    /// Creates the PDF pattern and necessary resources (Shadings, Functions, Soft Masks).
    ///
    /// Returns a tuple of (Pattern Name, optional Graphics State Name for transparency).
    pub fn create_pattern(
        &self,
        pdf: &mut PDF,
        resource_counter: &mut u32,
        posn: PosnPoints,
        size: DimsPoints,
        stroke_width: f64,
    ) -> Option<(String, Option<String>)> {
        if self.stops.len() < 2 {
            return None;
        }

        // 1. Determine Geometry Strategy
        let (shading_type, coords) = self.get_shading_params(posn, size, stroke_width);

        let pattern_name = format!("P{}", *resource_counter);
        *resource_counter += 1;

        let first = &self.stops[0].rgba;
        let last = &self.stops.last().unwrap().rgba;

        let extend = Arc::new(ArrayObject::new(Some(vec![
            Arc::new(BooleanObject::new(true)) as Arc<dyn PdfObject>,
            Arc::new(BooleanObject::new(true)),
        ])));

        // 2. Create Color Function (Type 2 - Exponential Interpolation)
        let color_func = create_interpolation_function(
            vec![first.red, first.green, first.blue],
            vec![last.red, last.green, last.blue],
        );
        let color_func_num = pdf.add_object(Box::new(color_func));

        // 3. Create Color Shading Dictionary
        let mut shading_dict = DictionaryObject::new(None);
        shading_dict.set(
            "ShadingType",
            Arc::new(NumberObject::from(shading_type as i64)),
        );
        shading_dict.set(
            "ColorSpace",
            Arc::new(NameObject::new("DeviceRGB".to_string())),
        );
        shading_dict.set("Coords", Arc::new(Array::new(Some(coords.clone()))));
        shading_dict.set(
            "Function",
            Arc::new(IndirectReference {
                metadata: Default::default(),
                id: color_func_num,
            }),
        );
        shading_dict.set("Extend", extend.clone());

        let shading_num = pdf.add_object(Box::new(shading_dict));

        // 4. Handle Transparency (Soft Mask)
        let has_transparency = first.alpha < 1.0 || last.alpha < 1.0;
        let gs_name = if has_transparency {
            let name = format!("GS{}", *resource_counter);
            *resource_counter += 1;

            // Alpha Interpolation Function
            let alpha_func = create_interpolation_function(vec![first.alpha], vec![last.alpha]);
            let alpha_func_num = pdf.add_object(Box::new(alpha_func));

            // Alpha Shading (DeviceGray)
            let mut alpha_shading = DictionaryObject::new(None);
            alpha_shading.set(
                "ShadingType",
                Arc::new(NumberObject::from(shading_type as i64)),
            );
            alpha_shading.set(
                "ColorSpace",
                Arc::new(NameObject::new("DeviceGray".to_string())),
            );
            alpha_shading.set("Coords", Arc::new(Array::new(Some(coords))));
            alpha_shading.set(
                "Function",
                Arc::new(IndirectReference {
                    metadata: Default::default(),
                    id: alpha_func_num,
                }),
            );
            alpha_shading.set("Extend", extend);

            let alpha_shading_num = pdf.add_object(Box::new(alpha_shading));

            // Create the Soft Mask group and ExtGState
            create_soft_mask_for_shading(pdf, alpha_shading_num, size.width, size.height);

            Some(name)
        } else {
            None
        };

        // 5. Create Pattern Dictionary
        let mut pattern_dict = DictionaryObject::typed("Pattern");
        pattern_dict.set("PatternType", Arc::new(NumberObject::from(2)));
        pattern_dict.set(
            "Shading",
            Arc::new(IndirectReference {
                metadata: Default::default(),
                id: shading_num,
            }),
        );

        pdf.add_object(Box::new(pattern_dict));

        Some((pattern_name, gs_name))
    }

    /// Calculates geometry parameters (Type and Coords) based on gradient kind.
    fn get_shading_params(
        &self,
        posn: PosnPoints,
        size: DimsPoints,
        stroke_width: f64,
    ) -> (u8, Vec<f64>) {
        match self.kind {
            GradientKind::Linear { angle } => {
                let math_angle = 90.0 - angle;
                let angle_rad = (math_angle as f64).to_radians();
                let cos = angle_rad.cos();
                let sin = angle_rad.sin();

                let cx = posn.x + size.width / 2.0;
                let cy = posn.y + size.height / 2.0;

                let half_len =
                    (size.width * cos.abs() + size.height * sin.abs()) / 2.0 + stroke_width;

                let x0 = cx - cos * half_len;
                let y0 = cy + sin * half_len;
                let x1 = cx + cos * half_len;
                let y1 = cy - sin * half_len;

                (2, vec![x0, y0, x1, y1])
            }
            GradientKind::Radial => {
                let cx = posn.x + size.width / 2.0;
                let cy = posn.y + size.height / 2.0;
                let radius = size.width.min(size.height) * 1.5;
                // [x0 y0 r0 x1 y1 r1]
                (3, vec![cx, cy, 0.0, cx, cy, radius])
            }
        }
    }
}

//--------------------------- Helpers ---------------------------//

/// Creates a Type 2 (Exponential Interpolation) Function dictionary.
fn create_interpolation_function(c0: Vec<f64>, c1: Vec<f64>) -> DictionaryObject {
    let mut dict = DictionaryObject::new(None);
    dict.set("FunctionType", Arc::new(NumberObject::from(2)));
    dict.set("Domain", Arc::new(Array::new(Some(vec![0.0, 1.0]))));
    dict.set("C0", Arc::new(Array::new(Some(c0))));
    dict.set("C1", Arc::new(Array::new(Some(c1))));
    dict.set("N", Arc::new(NumberObject::from(1))); // Linear interpolation
    dict
}

/// Orchestrates the creation of the Soft Mask (/SMask) object graph.
fn create_soft_mask_for_shading(pdf: &mut PDF, alpha_shading_num: usize, width: f64, height: f64) {
    // 1. Create Form XObject (Transparency Group)
    let mut form_extra = Vec::new();
    form_extra.push((
        "Type".to_string(),
        Arc::new(NameObject::new("XObject".to_string())) as Arc<dyn PdfObject>,
    ));
    form_extra.push((
        "Subtype".to_string(),
        Arc::new(NameObject::new("Form".to_string())),
    ));
    form_extra.push(("FormType".to_string(), Arc::new(NumberObject::from(1))));
    form_extra.push((
        "BBox".to_string(),
        Arc::new(Array::new(Some(vec![0.0, 0.0, width, height]))),
    ));

    let mut group_dict = DictionaryObject::new(None);
    group_dict.set("Type", Arc::new(NameObject::new("Group".to_string())));
    group_dict.set("S", Arc::new(NameObject::new("Transparency".to_string())));
    group_dict.set("CS", Arc::new(NameObject::new("DeviceGray".to_string())));
    form_extra.push(("Group".to_string(), Arc::new(group_dict)));

    let mut resources = DictionaryObject::new(None);
    let mut shading_res = DictionaryObject::new(None);
    shading_res.set(
        "Sh0",
        Arc::new(IndirectReference {
            metadata: Default::default(),
            id: alpha_shading_num,
        }),
    );
    resources.set("Shading", Arc::new(shading_res));
    form_extra.push(("Resources".to_string(), Arc::new(resources)));

    let form_stream =
        StreamObject::new().with_data(Some(vec![b"/Sh0 sh".to_vec()]), Some(form_extra));
    let form_number = pdf.add_object(Box::new(form_stream));

    // 2. Create Mask Dictionary
    let mut smask_dict = DictionaryObject::typed("Mask");
    smask_dict.set("S", Arc::new(NameObject::new("Luminosity".to_string())));
    smask_dict.set(
        "G",
        Arc::new(IndirectReference {
            metadata: Default::default(),
            id: form_number,
        }),
    );
    let smask_number = pdf.add_object(Box::new(smask_dict));

    // 3. Create ExtGState with the SMask
    let mut gs_dict = DictionaryObject::typed("ExtGState");
    gs_dict.set(
        "SMask",
        Arc::new(IndirectReference {
            metadata: Default::default(),
            id: smask_number,
        }),
    );
    pdf.add_object(Box::new(gs_dict));
}
