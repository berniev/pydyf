pub struct Trailer{}

impl Trailer{
    pub fn new() -> Self {
        Trailer{}
    }
}

impl Trailer {
    /*pub fn write(&self, stream: &&mut PdfStream) -> Result<(), std::io::Result<()>>{  // Write trailer
        /*stream.write_line(b"trailer")?;
        stream.write_line(b"<<")?;
        stream.write_line(format!("/Size {}", pdf.object_count()).as_bytes())?;
        stream.write_line(
            format!(
                "/Root {} 0 R",
                pdf.catalog.metadata.object_identifier.unwrap()
            )
                .as_bytes(),
        )?;

        if !pdf.info.values.is_empty() {
            stream.write_line(
                &format!("/Info {} 0 R", pdf.info.metadata.object_identifier.unwrap()).into_bytes(),
            )?;
        }

        if let Some(id_line) = Self::format_identifier(&pdf.objects, id_mode) {
            stream.write_line(&id_line)?;
        }

        stream.write_line(b">>")?;*/

        Ok(())
    }*/
}
