use crate::{NumberType, PdfError};
//---------------- PdfNumberObject -----------------

#[derive(Debug, Clone, PartialEq)]
pub struct PdfNumberObject {
    pub(crate) value: NumberType,
    pub(crate) object_number: Option<u64>,
    pub(crate) generation_number: Option<u16>,
}

impl PdfNumberObject {
    pub fn new(value: NumberType) -> Self {
        Self {
            value,
            object_number: None,
            generation_number: None,
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
