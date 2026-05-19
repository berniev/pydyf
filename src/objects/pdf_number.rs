use crate::object_ops::{Encode, PdfObject, Serialize};
use crate::PdfError;
use crate::version::Version;

//---------------- PdfNumberObject -----------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PdfNumberObject {
    Integer(i64),
    Real(f64),
}

impl PdfNumberObject {
    pub fn as_int(&self) -> i64 {
        match self {
            PdfNumberObject::Integer(i) => *i,
            PdfNumberObject::Real(f) => *f as i64,
        }
    }

    pub fn as_real(&self) -> f64 {
        match self {
            PdfNumberObject::Integer(i) => *i as f64,
            PdfNumberObject::Real(f) => *f,
        }
    }
}

//---------------- Primitive to concrete -----------------

impl From<i64> for PdfNumberObject {
    fn from(i: i64) -> Self { Self::Integer(i) }
}
impl From<f64> for PdfNumberObject {
    fn from(f: f64) -> Self { Self::Real(f) }
}
impl From<f32> for PdfNumberObject {
    fn from(f: f32) -> Self { Self::Real(f as f64) }
}
impl From<u32> for PdfNumberObject {
    fn from(v: u32) -> Self { Self::Integer(v as i64) }
}
impl From<i32> for PdfNumberObject {
    fn from(v: i32) -> Self { Self::Integer(v as i64) }
}

//---------------- impl's -----------------

impl Encode for PdfNumberObject {
    fn encode(&self, _version: Version) -> Result<Vec<u8>, PdfError> {
        let s = match self {
            PdfNumberObject::Integer(i) => i.to_string(),
            PdfNumberObject::Real(f) => format!("{:.4}", f)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string(),
        };
        Ok(s.into_bytes())
    }
}

impl Serialize for PdfNumberObject {}

//--------------------------- Box<dyn PdfObject> -------------------------//

impl From<PdfNumberObject> for Box<dyn PdfObject> {
    fn from(v: PdfNumberObject) -> Self { Box::new(v) }
}
impl From<i64> for Box<dyn PdfObject> {
    fn from(v: i64) -> Self { Box::new(PdfNumberObject::Integer(v)) }
}
impl From<f64> for Box<dyn PdfObject> {
    fn from(v: f64) -> Self { Box::new(PdfNumberObject::Real(v)) }
}
impl From<usize> for Box<dyn PdfObject> {
    fn from(v: usize) -> Self { Box::new(PdfNumberObject::Integer(v as i64)) }
}
impl From<u64> for Box<dyn PdfObject> {
    fn from(v: u64) -> Self { Box::new(PdfNumberObject::Integer(v as i64)) }
}
impl From<i32> for Box<dyn PdfObject> {
    fn from(v: i32) -> Self { Box::new(PdfNumberObject::Integer(v as i64)) }
}
impl From<f32> for Box<dyn PdfObject> {
    fn from(v: f32) -> Self { Box::new(PdfNumberObject::Real(v as f64)) }
}
impl From<u32> for Box<dyn PdfObject> {
    fn from(v: u32) -> Self { Box::new(PdfNumberObject::Integer(v as i64)) }
}
impl From<u8> for Box<dyn PdfObject> {
    fn from(v: u8) -> Self { Box::new(PdfNumberObject::Integer(v as i64)) }
}

//--------------------------- Tests -------------------------//

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object_ops::Encode;
    use crate::version::Version;

    #[test]
    fn encode_positive_integer() {
        let obj = PdfNumberObject::from(42);
        assert_eq!(obj.encode(Version::V1_5).unwrap(), b"42");
    }

    #[test]
    fn encode_negative_integer() {
        let obj = PdfNumberObject::from(-7);
        assert_eq!(obj.encode(Version::V1_5).unwrap(), b"-7");
    }

    #[test]
    fn encode_zero_integer() {
        let obj = PdfNumberObject::from(0);
        assert_eq!(obj.encode(Version::V1_5).unwrap(), b"0");
    }

    #[test]
    fn encode_real() {
        let obj = PdfNumberObject::from(3.14);
        assert_eq!(obj.encode(Version::V1_5).unwrap(), b"3.14");
    }

    #[test]
    fn encode_real_whole_number_strips_trailing() {
        let obj = PdfNumberObject::from(5.0);
        assert_eq!(obj.encode(Version::V1_5).unwrap(), b"5");
    }

    #[test]
    fn encode_negative_real() {
        let obj = PdfNumberObject::from(-0.5);
        assert_eq!(obj.encode(Version::V1_5).unwrap(), b"-0.5");
    }

    #[test]
    fn encode_real_precision_four_decimals() {
        // 1.23456 → formatted as "1.2346" (4 decimal places, trailing zeros stripped)
        let obj = PdfNumberObject::from(1.23456);
        assert_eq!(obj.encode(Version::V1_5).unwrap(), b"1.2346");
    }
}
