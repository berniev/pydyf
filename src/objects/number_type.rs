//---------------- NumberType -----------------

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PdfNumberType {
    Integer(i64),
    Real(f64),
}

impl From<u8> for PdfNumberType {
    fn from(i: u8) -> Self {
        Self::Integer(i as i64)
    }
}

impl From<usize> for PdfNumberType {
    fn from(i: usize) -> Self {
        Self::Integer(i as i64)
    }
}

impl From<u32> for PdfNumberType {
    fn from(i: u32) -> Self {
        Self::Integer(i as i64)
    }
}

impl From<u64> for PdfNumberType {
    fn from(i: u64) -> Self {
        Self::Integer(i as i64)
    }
}

impl From<i64> for PdfNumberType {
    fn from(i: i64) -> Self {
        Self::Integer(i)
    }
}

impl From<f64> for PdfNumberType {
    fn from(f: f64) -> Self {
        Self::Real(f)
    }
}

impl From<i32> for PdfNumberType {
    fn from(i: i32) -> Self {
        Self::Integer(i as i64)
    }
}

impl From<f32> for PdfNumberType {
    fn from(f: f32) -> Self {
        Self::Real(f as f64)
    }
}
