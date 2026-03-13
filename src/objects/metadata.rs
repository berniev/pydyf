use std::fmt;

//---------------- ObjectStatus -----------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectStatus {
    Free,  // deleted or never used
    InUse, // normal, active object
}

impl ObjectStatus {
    pub fn as_char(&self) -> char {
        match self {
            ObjectStatus::Free => 'f',
            ObjectStatus::InUse => 'n',
        }
    }
}

impl fmt::Display for ObjectStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

impl Default for ObjectStatus {
    fn default() -> Self {
        ObjectStatus::InUse
    }
}

//---------------- Generation -----------------

/// PDF generation number:
///     0 = original version
/// 65535 = special value for the free object 0
///     1+ = updated versions
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Generation {
    Root,
    Normal,
}

impl Generation {
    pub fn as_u16(&self) -> u16 {
        match self {
            Generation::Root => 65535,
            Generation::Normal => 0,
        }
    }
}

impl fmt::Display for Generation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_u16())
    }
}

//---------------- PdfMetadata -----------------


#[derive(Debug, Clone, PartialEq)]
pub struct PdfMetadata {
    pub object_identifier: Option<usize>, // None for unassigned objects
    pub generation_number: Generation,
    pub status: ObjectStatus,
}

impl PdfMetadata {
    pub fn new() -> Self {
        PdfMetadata {
            object_identifier: None,
            generation_number: Generation::Normal,
            status: ObjectStatus::InUse,
        }
    }

    pub fn new_free() -> Self {
        let mut pdf = PdfMetadata::new();
        pdf.status = ObjectStatus::Free;

        pdf
    }
}

impl Default for PdfMetadata {
    fn default() -> Self {
        Self::new()
    }
}
