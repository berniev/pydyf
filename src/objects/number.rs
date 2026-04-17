use crate::{NumberType, PdfError};

#[derive(Debug, Clone, PartialEq)]
pub struct PdfNumberObject {
    pub(crate) value: NumberType,
}

impl PdfNumberObject {
    pub fn new(value: NumberType) -> Self {
        Self {
            value,
        }
    }

    pub fn set_value<T: Into<NumberType>>(&mut self, value: T) {
        self.value = value.into();
    }

    pub fn as_int(&self) -> i64 {
        match self.value {
            NumberType::Integer(i) => i,
            NumberType::Real(f) => f as i64,
        }
    }

    pub fn as_real(&self) -> f64 {
        match self.value {
            NumberType::Integer(i) => i as f64,
            NumberType::Real(f) => f,
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, PdfError> {
        let mut arr = vec![];
        arr = match self.value {
            NumberType::Integer(i) => Vec::from(&*i.to_string().into_bytes()),
            NumberType::Real(f) => {
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

impl From<NumberType> for PdfNumberObject {
    fn from(value: NumberType) -> Self {
        Self::new(value)
    }
}

impl From<i64> for PdfNumberObject {
    fn from(i: i64) -> Self {
        Self::new(NumberType::Integer(i))
    }
}

impl From<f64> for PdfNumberObject {
    fn from(f: f64) -> Self {
        Self::new(NumberType::Real(f))
    }
}

impl From<i32> for PdfNumberObject {
    fn from(i: i32) -> Self {
        Self::new(NumberType::Integer(i as i64))
    }
}

impl From<f32> for PdfNumberObject {
    fn from(f: f32) -> Self {
        Self::new(NumberType::Real(f as f64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_positive_integer() {
        let obj = PdfNumberObject::new(NumberType::Integer(42));
        assert_eq!(obj.encode().unwrap(), b"42");
    }

    #[test]
    fn encode_negative_integer() {
        let obj = PdfNumberObject::new(NumberType::Integer(-7));
        assert_eq!(obj.encode().unwrap(), b"-7");
    }

    #[test]
    fn encode_zero_integer() {
        let obj = PdfNumberObject::new(NumberType::Integer(0));
        assert_eq!(obj.encode().unwrap(), b"0");
    }

    #[test]
    fn encode_real() {
        let obj = PdfNumberObject::new(NumberType::Real(3.14));
        assert_eq!(obj.encode().unwrap(), b"3.14");
    }

    #[test]
    fn encode_real_whole_number_strips_trailing() {
        let obj = PdfNumberObject::new(NumberType::Real(5.0));
        assert_eq!(obj.encode().unwrap(), b"5");
    }

    #[test]
    fn encode_negative_real() {
        let obj = PdfNumberObject::new(NumberType::Real(-0.5));
        assert_eq!(obj.encode().unwrap(), b"-0.5");
    }

    #[test]
    fn encode_real_precision_four_decimals() {
        // 1.23456 → formatted as "1.2346" (4 decimal places, trailing zeros stripped)
        let obj = PdfNumberObject::new(NumberType::Real(1.23456));
        assert_eq!(obj.encode().unwrap(), b"1.2346");
    }
}