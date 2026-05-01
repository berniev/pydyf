use std::fmt;
use std::io;

use crate::color::{CMYK, RGB, RGBA};
use crate::xref_ops::XRefError;

#[derive(Debug)]
pub enum PdfError {
    Io(io::Error),
    InvalidObjectReference(usize),
    InvalidArgument(String),
    CompressionError(String),
    CrossRef(XRefError),
    InvalidColorValue { val :f32 },
    InvalidFont(String),
    InvalidFunctionSpecification,
    InvalidRGB { rgb: RGB },
    InvalidRGBA { rgb: RGBA },
    InvalidCMYK { cmyk: CMYK },
    InvalidImage(String),
    StructureError(String),
    SerializeError(String),
    StreamError(String),
}

impl fmt::Display for PdfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PdfError::Io(err) => write!(f, "I/O error: {}", err),
            PdfError::InvalidObjectReference(num) => {
                write!(f, "Invalid object reference: {}", num)
            }
            PdfError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
            PdfError::CompressionError(msg) => write!(f, "Compression error: {}", msg),
            PdfError::InvalidFont(msg) => write!(f, "Invalid font: {}", msg),
            PdfError::InvalidColorValue { val } => {
                write!(f, "color value {} not in range 0.0..=1.0", val)
            }
            PdfError::InvalidRGB { rgb } => {
                write!(
                    f,
                    "Invalid color values: r={}, g={}, b={} (must be 0.0-1.0)",
                    rgb.r(),
                    rgb.g(),
                    rgb.b()
                )
            }
            PdfError::InvalidRGBA { rgb } => {
                write!(
                    f,
                    "Invalid color values: r={}, g={}, b={}, a={} (must be 0.0-1.0)",
                    rgb.r(),
                    rgb.g(),
                    rgb.b(),
                    rgb.a()
                )
            }
            PdfError::InvalidCMYK { cmyk } => {
                write!(
                    f,
                    "Invalid color values: {} (must all be 0.0-1.0)",
                    cmyk.as_string()
                )
            }
            PdfError::InvalidImage(msg) => write!(f, "Invalid image: {}", msg),
            PdfError::StructureError(msg) => write!(f, "PDF structure error: {}", msg),
            PdfError::SerializeError(msg) => write!(f, "Serialization error: {}", msg),
            PdfError::StreamError(msg) => write!(f, "Stream error: {}", msg),
            PdfError::CrossRef(err) => write!(f, "Cross-reference table error: {:?}", err),
            PdfError::InvalidFunctionSpecification => write!(f, "Invalid function specification"),
        }
    }
}

impl std::error::Error for PdfError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PdfError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for PdfError {
    fn from(err: io::Error) -> Self {
        PdfError::Io(err)
    }
}

impl From<XRefError> for PdfError {
    fn from(err: XRefError) -> Self {
        PdfError::CrossRef(err)
    }
}

pub type PdfResult<T> = Result<T, PdfError>;
