use crate::PdfError;
use crate::version::Version;
use std::fs::File;
use std::io::Write;

pub fn serialize(version: Version, file: &mut File) -> Result<(), PdfError> {
    let mut arr: Vec<u8> = vec![];
    arr.extend(b"%PDF-");
    arr.extend(version.as_bytes());
    arr.extend(b"\r\n");
    arr.extend("âãÏÓ\r\n".as_bytes());

    file.write_all(&arr).map_err(PdfError::Io)?;

    Ok(())
}
