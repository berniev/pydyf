use crate::util::{StreamString, f64_to_pdf_string};
use crate::{PdfArrayObject, PdfError};
use std::cmp::Ordering;
use std::fmt::{self, Display};

//------------------------ ColorSpace -------------------------------

pub enum ColorSpace {
    CMYK,
    Gray,
    RGB,
}

impl Display for ColorSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorSpace::RGB => f.write_str("RGB"),
            ColorSpace::CMYK => f.write_str("CMYK"),
            ColorSpace::Gray => f.write_str("Gray"),
        }
    }
}

impl ColorSpace {
    pub fn from_string(s: &str) -> Option<ColorSpace> {
        match s {
            "RGB" => Some(ColorSpace::RGB),
            "CMYK" => Some(ColorSpace::CMYK),
            "Gray" => Some(ColorSpace::Gray),
            _ => None,
        }
    }
}

pub enum ColorsInSpace {
    RGB(RGB),
    CMYK(CMYK),
    Gray(Color),
    None,
}
impl ColorsInSpace {
    pub fn as_pdf_array(&self) -> PdfArrayObject {
        let arr = match self {
            ColorsInSpace::RGB(rgb) => rgb.as_vec(),
            ColorsInSpace::CMYK(cmyk) => cmyk.as_vec(),
            ColorsInSpace::Gray(gray) => vec![gray.color],
            ColorsInSpace::None => vec![],
        };

        PdfArrayObject::from_vec_number(arr)
    }
}

//------------------------ Color -------------------------------

#[derive(Debug, Clone, Copy)]
pub struct Color {
    color: f32,
}

impl Color {
    pub fn new(value: f32) -> Result<Self, PdfError> {
        if !(0.0..=1.0).contains(&value) {
            return Err(PdfError::InvalidColorValue { val: value });
        }
        Ok(Color { color: value })
    }

    pub fn to_f32(&self) -> f32 {
        self.color
    }

    pub fn to_f64(&self) -> f64 {
        self.color as f64
    }

    pub fn as_string(&self) -> String {
        format!("{}", self.color)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", f64_to_pdf_string(self.color as f64))
    }
}

impl StreamString for Color {
    fn to_stream_string(&self) -> String {
        f64_to_pdf_string(self.color as f64)
    }
}

impl PartialEq<f32> for Color {
    fn eq(&self, other: &f32) -> bool {
        self.color == *other
    }
}

impl PartialOrd<f32> for Color {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        self.color.partial_cmp(other)
    }
}

//------------------------- RGB -------------------------------

#[derive(Debug, Clone, Copy)]
pub struct RGB {
    red: Color,
    green: Color,
    blue: Color,
}

impl RGB {
    pub fn new(red: Color, green: Color, blue: Color) -> Self {
        Self { red, green, blue }
    }

    pub fn as_vec(&self) -> Vec<f32> {
        vec![self.red.color, self.green.color, self.blue.color]
    }

    pub fn r(&self) -> Color {
        self.red
    }

    pub fn g(&self) -> Color {
        self.green
    }

    pub fn b(&self) -> Color {
        self.blue
    }

    pub const RED: RGB = RGB {
        red: Color { color: 1.0 },
        green: Color { color: 0.0 },
        blue: Color { color: 0.0 },
    };
    pub const ORANGE: RGB = RGB {
        red: Color { color: 1.0 },
        green: Color { color: 0.5 },
        blue: Color { color: 0.0 },
    };
    pub const YELLOW: RGB = RGB {
        red: Color { color: 1.0 },
        green: Color { color: 1.0 },
        blue: Color { color: 0.0 },
    };
    pub const GREEN: RGB = RGB {
        red: Color { color: 0.0 },
        green: Color { color: 1.0 },
        blue: Color { color: 0.0 },
    };
    pub const CYAN: RGB = RGB {
        red: Color { color: 0.0 },
        green: Color { color: 1.0 },
        blue: Color { color: 1.0 },
    };
    pub const BLUE: RGB = RGB {
        red: Color { color: 0.0 },
        green: Color { color: 0.0 },
        blue: Color { color: 1.0 },
    };
    pub const MAGENTA: RGB = RGB {
        red: Color { color: 1.0 },
        green: Color { color: 0.0 },
        blue: Color { color: 1.0 },
    };
    pub const PURPLE: RGB = RGB {
        red: Color { color: 1.0 },
        green: Color { color: 0.0 },
        blue: Color { color: 0.5 },
    };
    pub const BROWN: RGB = RGB {
        red: Color { color: 0.6 },
        green: Color { color: 0.4 },
        blue: Color { color: 0.2 },
    };
    pub const GREY: RGB = RGB {
        red: Color { color: 0.5 },
        green: Color { color: 0.5 },
        blue: Color { color: 0.5 },
    };
    pub const WHITE: RGB = RGB {
        red: Color { color: 1.0 },
        green: Color { color: 1.0 },
        blue: Color { color: 1.0 },
    };
    pub const BLACK: RGB = RGB {
        red: Color { color: 0.0 },
        green: Color { color: 0.0 },
        blue: Color { color: 0.0 },
    };
}

impl StreamString for RGB {
    fn to_stream_string(&self) -> String {
        format!(
            "{} {} {}",
            f64_to_pdf_string(self.red.to_f64()),
            f64_to_pdf_string(self.green.to_f64()),
            f64_to_pdf_string(self.blue.to_f64())
        )
    }
}

//------------------------ RGBA -------------------------------

#[derive(Debug, Clone, Copy)]
pub struct RGBA {
    red: Color,
    green: Color,
    blue: Color,
    alpha: Color,
}

impl RGBA {
    pub fn new(red: Color, green: Color, blue: Color, alpha: Color) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn as_vec(&self) -> Vec<f32> {
        vec![
            self.red.color,
            self.green.color,
            self.blue.color,
            self.alpha.color,
        ]
    }

    pub fn has_transparency(&self) -> bool {
        self.alpha.color < 1.0
    }

    pub fn r(&self) -> Color {
        self.red
    }

    pub fn g(&self) -> Color {
        self.green
    }

    pub fn b(&self) -> Color {
        self.blue
    }

    pub fn a(&self) -> Color {
        self.alpha
    }
}

impl StreamString for RGBA {
    fn to_stream_string(&self) -> String {
        format!(
            "{} {} {} {}",
            f64_to_pdf_string(self.red.to_f64()),
            f64_to_pdf_string(self.green.to_f64()),
            f64_to_pdf_string(self.blue.to_f64()),
            f64_to_pdf_string(self.alpha.to_f64())
        )
    }
}

//------------------------ CMYK -------------------------------

#[derive(Debug, Clone, Copy)]
pub struct CMYK {
    cyan: Color,
    magenta: Color,
    yellow: Color,
    black: Color,
}

impl CMYK {
    pub fn new(cyan: Color, magenta: Color, yellow: Color, black: Color) -> Self {
        Self {
            cyan,
            magenta,
            yellow,
            black,
        }
    }

    pub fn as_vec(&self) -> Vec<f32> {
        vec![self.cyan.color, self.magenta.color, self.yellow.color, self.black.color]
    }

    pub fn as_string(&self) -> String {
        format!(
            "{} {} {} {}",
            f64_to_pdf_string(self.cyan.to_f64()),
            f64_to_pdf_string(self.magenta.to_f64()),
            f64_to_pdf_string(self.yellow.to_f64()),
            f64_to_pdf_string(self.black.to_f64())
        )
    }
}

impl StreamString for CMYK {
    fn to_stream_string(&self) -> String {
        self.as_string()
    }
}
