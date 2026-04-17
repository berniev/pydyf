use crate::object_ops::ObjectOps;
use std::cell::RefCell;
use std::rc::Rc;

/// Trait for PDF objects that can be indirect (assigned an object number).
pub trait AssignObjectNumber: Sized {
    fn set_object_number(&mut self, value: u64);

    /// Allocate the next object number from the shared ObjectOps and assign it.
    fn with_next_object_number(mut self, object_ops: &Rc<RefCell<ObjectOps>>) -> Self {
        let num = object_ops.borrow_mut().next_object_number();
        self.set_object_number(num);
        self
    }
}