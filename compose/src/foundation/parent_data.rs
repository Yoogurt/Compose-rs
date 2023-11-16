use std::cell::{Ref, RefMut};
use std::fmt::Debug;
use crate::foundation::measurable::Measurable;

pub trait ParentData: Debug {}

pub(crate) trait ExtractParentData {
    fn cast<T>(&self) -> Option<Ref<T>> where T: Sized + 'static;
    fn cast_mut<T>(&self) -> Option<RefMut<T>> where T: Sized + 'static;
}

impl<T> ExtractParentData for T where T: ?Sized + Measurable {
    fn cast<R>(&self) -> Option<Ref<R>> where R: Sized + 'static {
        self.get_parent_data_ref()
            .and_then(|parent_data| Ref::filter_map(parent_data.borrow(), |parent_data| {
                parent_data.downcast_ref::<R>()
            }).ok())
    }

    fn cast_mut<R>(&self) -> Option<RefMut<R>> where R: Sized + 'static {
        self.get_parent_data_ref()
            .and_then(|parent_data| RefMut::filter_map(parent_data.borrow_mut(), |parent_data| {
                parent_data.downcast_mut::<R>()
            }).ok())
    }
}