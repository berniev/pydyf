pub enum PageSize {
    A4,
    Letter,
    Legal,
    A3,
    Custom(f64, f64), // width, height in points
}

impl PageSize {
    /// Returns the [width, height] in PDF points (1/72 inch).
    /// Returns (0.0, 0.0) for negative dimensions in Custom.
    pub fn dimensions(&self) -> (f64, f64) {
        match self {
            PageSize::A4 => (595.0, 842.0),
            PageSize::Letter => (612.0, 792.0),
            PageSize::Legal => (612.0, 1008.0),
            PageSize::A3 => (842.0, 1191.0),
            PageSize::Custom(w, h) => (w.max(0.0), h.max(0.0)),
        }
    }

    /// Helper to generate the PDF MediaBox string: [0 0 width height]
    pub fn to_mediabox(&self) -> Vec<u8> {
        let (w, h) = self.dimensions();
        format!("[0 0 {} {}]", w, h).into_bytes()
    }
}