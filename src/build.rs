use std::rc::Rc;

use crate::PdfObject;

/// Trait for types that can build a PDF object representation.
///
/// This trait provides a unified interface for building PDF objects from
/// various types (primitives, domain types, etc.).
///
/// # Examples
///
/// ```
/// use pydyf::NumberObject;
///
/// // Build a number object
/// let num_obj = NumberObject::build(42);
/// let real_obj = NumberObject::build(3.14);
/// ```
pub trait Build {
    /// Build a PDF object representation of this value.
    fn build(self) -> Rc<dyn PdfObject>;
}
