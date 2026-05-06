use crate::encoding::f64_to_pdf_string;
use crate::objects::pdf_number::PdfNumberObject;
use crate::{PdfNumberType, PdfArrayObject};
//------------------------- ToPdf -----------------------------//

pub trait StreamString {
    fn to_stream_string(&self) -> String;
}

//------------------------ Posn -------------------------------//

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Posn {
    pub x: f64,
    pub y: f64, // In pdf zero is at the bottom
}

impl StreamString for Posn {
    fn to_stream_string(&self) -> String {
        format!("{} {}", f64_to_pdf_string(self.x), f64_to_pdf_string(self.y))
    }
}

//------------------------ Line -------------------------------//

#[derive(Clone)]
pub struct Line {
    pub start: Posn,
    pub end: Posn,
}
impl Line{
    pub fn new(start: Posn, end: Posn) -> Self {
        Self { start, end }
    }

    pub fn as_pdf_array_object(&self) -> PdfArrayObject {
        let mut arr = PdfArrayObject::new();
        arr.push(self.start.x);
        arr.push(self.start.y);
        arr.push(self.end.x);
        arr.push(self.end.y);

        arr
    }
}
//------------------------ Dims -------------------------------//

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dims {
    pub width: f64,
    pub height: f64,
}

impl Dims {
    pub const fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl StreamString for Dims {
    fn to_stream_string(&self) -> String {
        format!(
            "{} {}",
            f64_to_pdf_string(self.width),
            f64_to_pdf_string(self.height),
        )
    }
}

//------------------------ Rect -------------------------------//

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub x1: f64, // lower-left x
    pub y1: f64, // lower-left y
    pub x2: f64, // upper-right x
    pub y2: f64, // upper-right y
}

impl Rectangle {
    pub fn as_pdf_array_object(&self) -> PdfArrayObject {
        let mut arr = PdfArrayObject::new();
        arr.push(PdfNumberObject::new(PdfNumberType::from(self.x1)));
        arr.push(PdfNumberObject::new(PdfNumberType::from(self.y1)));
        arr.push(PdfNumberObject::new(PdfNumberType::from(self.x2)));
        arr.push(PdfNumberObject::new(PdfNumberType::from(self.y2)));

        arr
    }
}

impl StreamString for Rectangle {
    fn to_stream_string(&self) -> String {
        format!(
            "{} {} {} {}",
            f64_to_pdf_string(self.x1),
            f64_to_pdf_string(self.y1),
            f64_to_pdf_string(self.x2),
            f64_to_pdf_string(self.y2),
        )
    }
}

//------------------------ Matrix -------------------------------

/// PDF transformation matrix [a b c d e f]
/// | a  b  0 |
/// | c  d  0 |
/// | e  f  1 |
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

impl Matrix {
    /*    fn as_vec(&self) -> Vec<f64> {
            vec![self.a, self.b, self.c, self.d, self.e, self.f]
        }
    */
    pub fn as_pdf_array(&self) -> PdfArrayObject {
        let mut arr = PdfArrayObject::new();
        arr.push(self.a);
        arr.push(self.b);
        arr.push(self.c);
        arr.push(self.d);
        arr.push(self.e);
        arr.push(self.f);

        arr
    }
}

impl StreamString for Matrix {
    fn to_stream_string(&self) -> String {
        format!(
            "{} {} {} {} {} {}",
            f64_to_pdf_string(self.a),
            f64_to_pdf_string(self.b),
            f64_to_pdf_string(self.c),
            f64_to_pdf_string(self.d),
            f64_to_pdf_string(self.e),
            f64_to_pdf_string(self.f),
        )
    }
}

//------------------------ EvenOdd -------------------------------//

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindingRule {
    NonZero,
    EvenOdd,
}

//------------------- CompressionMethod ----------------------------//

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompressionMethod {
    None,
    Flate,
}

impl CompressionMethod {
    pub fn to_string(&self) -> String {
        match self {
            CompressionMethod::Flate => "/A85 /Fl".to_string(),
            CompressionMethod::None => "/A85".to_string(),
        }
    }
}
//--------------------- StrokeOrFill -----------------------------//

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StrokeOrFill {
    Stroke,
    Fill,
}
