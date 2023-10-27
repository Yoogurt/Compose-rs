use std::cell::RefCell;
use std::rc::Rc;

pub(crate) trait WrapWithRcRefCell {
    fn wrap_with_rc_refcell(self) -> Rc<RefCell<Self>>;
}

impl<T> WrapWithRcRefCell for T {
    #[inline]
    fn wrap_with_rc_refcell(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }
}