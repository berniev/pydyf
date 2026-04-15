/// Functions
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
use crate::{PdfArrayObject, PdfDictionaryObject, PdfStreamObject};

//--------------------------- FunctionType ---------------------------//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionType {
    Sampled = 0,
    Exponential = 2,
    Stitching = 3,
    PostScript = 4,
}

impl FunctionType {
    pub fn as_str(&self) -> &str {
        match self {
            FunctionType::Sampled => "Sampled",
            FunctionType::Exponential => "Exponential",
            FunctionType::Stitching => "Stitching",
            FunctionType::PostScript => "PostScript",
        }
    }
}

//--------------------------- Function ---------------------------//

fn make_func_dict(
    func_type: FunctionType,
    domain: PdfArrayObject,
    range: Option<PdfArrayObject>,
) -> PdfDictionaryObject {
    let mut dict = PdfDictionaryObject::new();
    dict.add("FunctionType", func_type as i64);
    dict.add("Domain", domain);
    if let Some(range) = range {
        dict.add("Range", range);
    }

    dict
}

pub struct Function0Sampled {
    pub stream: PdfStreamObject,
}

impl Function0Sampled {
    pub fn new(
        domain: PdfArrayObject,
        range: PdfArrayObject,
        size: PdfArrayObject,
        bits_per_sample: u8,
        code: Vec<u8>,
    ) -> Self {
        let mut dict = make_func_dict(FunctionType::Sampled, domain, Some(range));
        dict.add("Size", size);
        assert!(
            matches!(bits_per_sample, 1 | 2 | 4 | 8 | 12 | 16 | 24 | 32),
            "BitsPerSample must be one of: 1, 2, 4, 8, 12, 16, 24, 32"
        );
        dict.add("BitsPerSample", bits_per_sample);
        let stream = PdfStreamObject::new().with_dict_and_content(dict, code);

        Self { stream }
    }

    pub fn with_order(mut self, order: u8) -> Self {
        assert!(
            order == 1 || order == 3,
            "Order must be 1 (linear) or 3 (cubic spline)"
        );
        self.stream.dict.add("Order", order);

        self
    }

    pub fn with_encode(mut self, encode: PdfArrayObject) -> Self {
        self.stream.dict.add("Encode", encode);

        self
    }

    pub fn with_decode(mut self, decode: PdfArrayObject) -> Self {
        self.stream.dict.add("Decode", decode);

        self
    }
}

pub struct Function2Exponential {
    pub dictionary: PdfDictionaryObject,
}

impl Function2Exponential {
    pub fn new(domain: PdfArrayObject, interpolation_exponent: f64) -> Self {
        let mut func = Function2Exponential {
            dictionary: make_func_dict(FunctionType::Exponential, domain, None),
        };
        func.dictionary.add("N", interpolation_exponent);

        func
    }

    pub fn with_values_at_start(mut self, values_at_start: PdfArrayObject) -> Self {
        self.dictionary.add("C0", values_at_start);

        self
    }

    pub fn with_values_at_end(mut self, values_at_end: PdfArrayObject) -> Self {
        self.dictionary.add("C1", values_at_end);

        self
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
    ) -> Self {
        let mut func = Function3Stitching {
            dictionary: make_func_dict(FunctionType::Stitching, domain, None),
        };
        func.dictionary.add("Functions", functions);
        func.dictionary.add("Bounds", bounds);
        func.dictionary.add("Encode", encode);

        func
    }
}

pub struct Function4PostScript {
    pub stream: PdfStreamObject,
}

impl Function4PostScript {
    pub fn new(domain: PdfArrayObject, range: PdfArrayObject, code: Vec<u8>) -> Self {
        let dict = make_func_dict(FunctionType::PostScript, domain, Some(range));
        let stream = PdfStreamObject::new().with_dict_and_content(dict,code);

        Self { stream }
    }
}
