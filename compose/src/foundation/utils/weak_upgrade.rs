use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub(crate) trait WeakUpdater<T: ?Sized> {
    fn try_upgrade(&self) -> Option<Rc<RefCell<T>>>;
}

impl<T> WeakUpdater<T> for Option<Weak<RefCell<T>>>
    where
        T: ?Sized,
{
    fn try_upgrade(&self) -> Option<Rc<RefCell<T>>> {
        match self {
            Some(result) => result.upgrade(),
            None => None,
        }
    }
}
