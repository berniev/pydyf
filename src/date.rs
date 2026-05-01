use crate::PdfError;

pub enum OffsetCode {
    P,
    M,
    Z,
}
impl OffsetCode {
    pub fn to_pdf_string(&self) -> String {
        match self {
            OffsetCode::P => "+",
            OffsetCode::M => "-",
            OffsetCode::Z => "Z",
        }
        .to_string()
    }
}

pub struct Date {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    offset_code: OffsetCode,
    offset_hours: u8,
    offset_minutes: u8,
}

impl Date {
    pub fn new(
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        offset_code: OffsetCode,
        offset_hours: u8,
        offset_minutes: u8,
    ) -> Result<Self, PdfError> {
        Self::check("year", year, 1, 9999)?;
        Self::check("month", month, 1, 12)?;
        Self::check("day", day, 1, 31)?;
        Self::check("hour", hour, 0, 23)?;
        Self::check("minute", minute, 0, 59)?;
        Self::check("second", second, 0, 59)?;
        Self::check("offset_hours", offset_hours, 0, 11)?;
        Self::check("offset_minutes", offset_minutes, 0, 59)?;
        Ok(Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            offset_code,
            offset_hours,
            offset_minutes,
        })
    }

    fn check<T: PartialOrd + std::fmt::Display>(
        name: &str,
        val: T,
        min: T,
        max: T,
    ) -> Result<(), PdfError> {
        if val < min || val > max {
            return Err(PdfError::InvalidArgument(format!(
                "{} {} out of range {}-{}",
                name, val, min, max
            )));
        }
        Ok(())
    }

    pub fn to_pdf_string(&self) -> String {
        format!(
            "D:{:04}{:02}{:02}{:02}{:02}{:02}{}{:02}'{:02}",
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
            self.offset_code.to_pdf_string(),
            self.offset_hours,
            self.offset_minutes
        )
    }
}
