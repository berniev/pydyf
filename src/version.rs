//--------------------------- Version -------------------------

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub enum Version {
    V1_4,
    #[default]
    V1_5,
    V1_6,
    V1_7,
}

impl Version {
    pub fn as_str(&self) -> &str {
        match self {
            Version::V1_4 => "1.4",
            Version::V1_5 => "1.5",
            Version::V1_6 => "1.6",
            Version::V1_7 => "1.7",
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}
