use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub struct SnapShotValue<T> {
    pub(crate) value: Rc<RefCell<T>>,
}

impl<'a, T> SnapShotValue<T> {
    pub fn borrow(&self) -> Ref<'_, T> {
        self.value.borrow()
    }

    pub fn borrow_mut(&mut self) -> RefMut<'_, T> {
        self.value.borrow_mut()
    }

    pub(crate) fn new(data: Rc<RefCell<T>>) -> Self {
        Self {
            value: data
        }
    }
}