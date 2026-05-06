use crate::{PdfNumberType, PdfError};
use crate::version::Version;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PdfNumberObject {
    pub(crate) value: PdfNumberType,
}

impl PdfNumberObject {
    pub fn new(value: PdfNumberType) -> Self {
        Self {
            value,
        }
    }

    pub fn set_value<T: Into<PdfNumberType>>(&mut self, value: T) {
        self.value = value.into();
    }

    pub fn as_int(&self) -> i64 {
        match self.value {
            PdfNumberType::Integer(i) => i,
            PdfNumberType::Real(f) => f as i64,
        }
    }

    pub fn as_real(&self) -> f64 {
        match self.value {
            PdfNumberType::Integer(i) => i as f64,
            PdfNumberType::Real(f) => f,
        }
    }

    pub fn encode(&self, _version: Version) -> Result<Vec<u8>, PdfError> {
        let mut arr = vec![];
        arr = match self.value {
            PdfNumberType::Integer(i) => Vec::from(&*i.to_string().into_bytes()),
            PdfNumberType::Real(f) => {
                Vec::from(
                    &*format!("{:.4}", f) // use a reasonable precision
                        .trim_end_matches('0')
                        .trim_end_matches('.')
                        .to_string()
                        .into_bytes(),
                )
            }
        };

        Ok(arr)
    }
}

impl From<PdfNumberType> for PdfNumberObject {
    fn from(value: PdfNumberType) -> Self {
        Self::new(value)
    }
}

impl From<i64> for PdfNumberObject {
    fn from(i: i64) -> Self {
        Self::new(PdfNumberType::Integer(i))
    }
}

impl From<f64> for PdfNumberObject {
    fn from(f: f64) -> Self {
        Self::new(PdfNumberType::Real(f))
    }
}

impl From<i32> for PdfNumberObject {
    fn from(i: i32) -> Self {
        Self::new(PdfNumberType::Integer(i as i64))
    }
}

impl From<f32> for PdfNumberObject {
    fn from(f: f32) -> Self {
        Self::new(PdfNumberType::Real(f as f64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_positive_integer() {
        let obj = PdfNumberObject::new(PdfNumberType::Integer(42));
        assert_eq!(obj.encode(Version::V1_5).unwrap(), b"42");
    }

    #[test]
    fn encode_negative_integer() {
        let obj = PdfNumberObject::new(PdfNumberType::Integer(-7));
        assert_eq!(obj.encode(Version::V1_5).unwrap(), b"-7");
    }

    #[test]
    fn encode_zero_integer() {
        let obj = PdfNumberObject::new(PdfNumberType::Integer(0));
        assert_eq!(obj.encode(Version::V1_5).unwrap(), b"0");
    }

    #[test]
    fn encode_real() {
        let obj = PdfNumberObject::new(PdfNumberType::Real(3.14));
        assert_eq!(obj.encode(Version::V1_5).unwrap(), b"3.14");
    }

    #[test]
    fn encode_real_whole_number_strips_trailing() {
        let obj = PdfNumberObject::new(PdfNumberType::Real(5.0));
        assert_eq!(obj.encode(Version::V1_5).unwrap(), b"5");
    }

    #[test]
    fn encode_negative_real() {
        let obj = PdfNumberObject::new(PdfNumberType::Real(-0.5));
        assert_eq!(obj.encode(Version::V1_5).unwrap(), b"-0.5");
    }

    #[test]
    fn encode_real_precision_four_decimals() {
        // 1.23456 → formatted as "1.2346" (4 decimal places, trailing zeros stripped)
        let obj = PdfNumberObject::new(PdfNumberType::Real(1.23456));
        assert_eq!(obj.encode(Version::V1_5).unwrap(), b"1.2346");
    }
}