//--------------------------- Version -------------------------

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub enum PdfVersion {
    #[default]
    Auto,
    V1_4,
    V1_5,
}

impl PdfVersion {
    pub fn as_str(&self) -> &str {
        match self {
            PdfVersion::Auto => "1.7",
            PdfVersion::V1_4 => "1.4",
            PdfVersion::V1_5 => "1.5",
        }
    }
}

