use crate::PdfArrayObject;
use crate::util::Dims;

//--------------------------- PageSize ---------------------------//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum PageSize {
    A0,
    A1,
    A2,
    A3,
    #[default]
    A4,
    A5,
    Letter,
    Legal,
    CustomPoints(Dims),
    CustomInches(Dims),
    CustomMm(Dims),
}
const MM_TO_POINTS: f64 = 2.8346456693;
const IN_TO_POINTS: f64 = 72.0;

impl PageSize {
    pub const fn dims_points(&self) -> Dims {
        use PageSize::*;
        match self {
            A0 => Dims::new(841.0 * MM_TO_POINTS, 1189.0 * MM_TO_POINTS),
            A1 => Dims::new(594.0 * MM_TO_POINTS, 841.0 * MM_TO_POINTS),
            A2 => Dims::new(420.0 * MM_TO_POINTS, 594.0 * MM_TO_POINTS),
            A3 => Dims::new(297.0 * MM_TO_POINTS, 420.0 * MM_TO_POINTS),
            A4 => Dims::new(210.0 * MM_TO_POINTS, 297.0 * MM_TO_POINTS),
            A5 => Dims::new(148.0 * MM_TO_POINTS, 210.0 * MM_TO_POINTS),
            Letter => Dims::new(8.5 * IN_TO_POINTS, 11.0 * IN_TO_POINTS),
            Legal => Dims::new(8.5 * IN_TO_POINTS, 14.0 * IN_TO_POINTS),
            CustomPoints(dims) => Dims::new(dims.width.max(0.0), dims.height.max(0.0)),
            CustomInches(dims) => Dims::new(
                dims.width.max(0.0) * IN_TO_POINTS,
                dims.height.max(0.0) * IN_TO_POINTS,
            ),
            CustomMm(dims) => Dims::new(
                dims.width.max(0.0) * MM_TO_POINTS,
                dims.height.max(0.0) * MM_TO_POINTS,
            ),
        }
    }

    pub fn to_rect(&self) -> PdfArrayObject {
        let Dims {width, height} = self.dims_points();
        let mut arr = PdfArrayObject::new();
        arr.push(0.0);
        arr.push(0.0);
        arr.push(width);
        arr.push(height);

        arr
    }
}
