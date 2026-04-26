//--------------------------- Version -------------------------

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub enum Version {
    V1_0,
    V1_1,
    V1_2,
    V1_3,
    V1_4,
    #[default]
    V1_5,
    V1_6,
    V1_7,
    V1_7_1,
    V1_7_3,
    V1_7_5,
    V1_7_6,
    V1_7_8,
    V2_2017,
    V2_2020
}

impl Version {
    pub fn as_str(&self) -> &str {
        use Version::*;
        match self {
            V1_0 => "1.0",
            V1_1 => "1.1",
            V1_2 => "1.2",
            V1_3 => "1.3",
            V1_4 => "1.4",
            V1_5 => "1.5",
            V1_6 => "1.6",
            V1_7 => "1.7",
            V1_7_1 => "1.7.1",
            V1_7_3 => "1.7.3",
            V1_7_5 => "1.7.5",
            V1_7_6 => "1.7.6",
            V1_7_8 => "1.7.8",
            V2_2017 => "2.2017",
            V2_2020 => "2.2020",
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}
