use crate::PdfIndirectObject;

pub trait PdfObject : 'static
{
    fn serialise(&mut self) -> Vec<u8>;

    fn boxed(self) -> Box<dyn PdfObject>
    where
        Self: Sized,
    {
        Box::new(self)
    }

    fn indirect_normal(&self, obj_num:usize) -> PdfIndirectObject {
        PdfIndirectObject::new_standard(obj_num);
    }
}
