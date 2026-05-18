use crate::object_number::ObjectNumber;
/// Functions
///
/// Function object may be a dictionary or a stream depending on the type of function.
///
/// Additional entries for type 0:
/// Key name        Type       Reqd  Value
/// ==============  =========  ====  ==============================================
/// Size            array      Reqd  Number of samples per dimension
/// BitsPerSample   integer    Reqd  Number of bits per sample
/// Order           integer    Opt   Interpolation: 1 = linear, 3 = cubic spline
/// Encode          array      Opt   Array of sample values (e.g., 0-255 for 8-bit)
/// Decode          array      Opt   Array of sample values (e.g., 0-255 for 8-bit)
/// -other-         (various)  Opt   attributes that provide sample values Table 5
/// ===============================================================================
///
/// Additional entries for type 2:
/// Key name        Type       Reqd  Value
/// ==============  =========  ====  ==============================================
/// C0              array      Opt   result when x = 0.0. def [0.0]
/// C1              array      Opt   result when x = 1.0. def [1.0]
/// N               number     Reqd  Interpolation exponent
/// ===============================================================================
///
/// Additional entries for type 3:
/// Key name        Type       Reqd  Value
/// ==============  =========  ====  ==============================================
/// Functions       array      Reqd  Array of sub-functions
/// Bounds          array      Reqd  Array of input values where sub-functions end
/// Encode          array      Reqd  Array of input values where sub-functions begin
/// ===============================================================================
///
/// Operators in type 4 functions:
/// Operator Type   Operators
/// ==============  ==============================================================
/// Arithmetic      abs add atan ceiling cos cvi cvr div exp floor idiv ln log mod
///                 mul neg round sin sqrt sub truncate
///
/// Relational      |
/// Bool            | and  eq false ge gt le lt ne not or true xor
/// Bitwisebitshift |
///
/// Conditional     if ifelse
///
/// Stack           copy dup exch index pop roll
/// ===============================================================================
///
use crate::{PdfArrayObject, PdfDictionaryObject, PdfError, PdfStreamObject};
//--------------------------- FunctionType ---------------------------//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionType {
    Sampled = 0,
    Exponential = 2,
    Stitching = 3,
    PostScript = 4,
}

//--------------------------- Function ---------------------------//

pub enum OrderType {
    Linear = 1,
    CubicSpline = 3,
}

pub struct Function0Sampled {
    pub stream: PdfStreamObject,
}
impl Function0Sampled {
    pub fn new(
        object_number: ObjectNumber,
        domain: PdfArrayObject,
        range: PdfArrayObject,
        size: PdfArrayObject,
        bits_per_sample: u32,
        code: Vec<u8>,
    ) -> Result<Self, PdfError> {
        if !matches!(bits_per_sample, 1 | 2 | 4 | 8 | 12 | 16 | 24 | 32) {
            return Err(PdfError::StructureError(format!(
                "BitsPerSample must be 1, 2, 4, 8, 12, 16, 24, or 32, got {}",
                bits_per_sample
            )));
        }
        let mut stream = PdfStreamObject::new(object_number).with_content(code);
        stream
            .dict
            .add("FunctionType", FunctionType::Sampled as i64)?;
        stream.dict.add("Domain", domain)?;
        stream.dict.add("Size", size)?;
        stream.dict.add("Range", range)?;
        stream.dict.add("BitsPerSample", bits_per_sample as i64)?;

        Ok(Self { stream })
    }

    pub fn with_order(mut self, order: OrderType) -> Result<Self, PdfError> {
        self.stream.dict.add("Order", order as i64)?;

        Ok(self)
    }

    pub fn with_encode(mut self, encode: PdfArrayObject) -> Result<Self, PdfError> {
        self.stream.dict.add("Encode", encode)?;

        Ok(self)
    }

    pub fn with_decode(mut self, decode: PdfArrayObject) -> Result<Self, PdfError> {
        self.stream.dict.add("Decode", decode)?;

        Ok(self)
    }
}

pub struct Function2Exponential {
    pub dictionary: PdfDictionaryObject,
}
impl Function2Exponential {
    pub fn new(domain: PdfArrayObject, interpolation_exponent: f64) -> Result<Self, PdfError> {
        let mut dictionary = PdfDictionaryObject::new();
        dictionary.add("Function Type", FunctionType::Exponential as i64)?;
        dictionary.add("Domain", domain)?;
        dictionary.add("N", interpolation_exponent)?;

        Ok(Self { dictionary })
    }

    pub fn with_range(mut self, range: PdfArrayObject) -> Result<Self, PdfError> {
        self.dictionary.add("Range", range)?;

        Ok(self)
    }

    pub fn with_values_at_start(
        mut self,
        values_at_start: PdfArrayObject,
    ) -> Result<Self, PdfError> {
        self.dictionary.add("C0", values_at_start)?;

        Ok(self)
    }

    pub fn with_values_at_end(mut self, values_at_end: PdfArrayObject) -> Result<Self, PdfError> {
        self.dictionary.add("C1", values_at_end)?;

        Ok(self)
    }
}

pub struct Function3Stitching {
    pub dictionary: PdfDictionaryObject,
}
impl Function3Stitching {
    pub fn new(
        functions: PdfArrayObject,
        domain: PdfArrayObject,
        bounds: PdfArrayObject,
        encode: PdfArrayObject,
    ) -> Result<Self, PdfError> {
        let mut dictionary = PdfDictionaryObject::new();
        dictionary.add("FunctionType", FunctionType::Stitching as i64)?;
        dictionary.add("Functions", functions)?;
        dictionary.add("Domain", domain)?;
        dictionary.add("Bounds", bounds)?;
        dictionary.add("Encode", encode)?;

        Ok(Self { dictionary })
    }

    pub fn with_range(mut self, range: PdfArrayObject) -> Result<Self, PdfError> {
        self.dictionary.add("Range", range)?;

        Ok(self)
    }
}

pub struct Function4PostScript {
    pub stream: PdfStreamObject,
}
impl Function4PostScript {
    pub fn new(
        object_number: ObjectNumber,
        domain: PdfArrayObject,
        range: PdfArrayObject,
        code: Vec<u8>,
    ) -> Result<Self, PdfError> {
        let mut stream = PdfStreamObject::new(object_number).with_content(code);
        stream
            .dict
            .add("FunctionType", FunctionType::PostScript as i64)?;
        stream.dict.add("Domain", domain)?;
        stream.dict.add("Range", range)?;

        Ok(Self { stream })
    }
}
