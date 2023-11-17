use std::any::Any;
use std::fmt::Debug;

use auto_delegate::delegate;

use crate::foundation::measurable::Measurable;

pub trait ParentData: Debug {}

#[delegate]
pub trait ParentDataGenerator {
    fn generate_parent_data(&self) -> Option<Box<dyn Any>>;
}

pub(crate) trait ExtractParentData {
    fn cast<T>(&self) -> Option<&T> where T: Sized + 'static;
    // fn cast_mut<T>(&self) -> Option<&mut Box<T>> where T: Sized + 'static;
}

impl<T> ExtractParentData for T where T: ?Sized + Measurable {
    fn cast<R>(&self) -> Option<&R> where R: Sized + 'static {
        self.get_parent_data()
            .and_then(|parent_data| {
                parent_data.downcast_ref()
            })
    }

    // fn cast_mut<R>(&self) -> Option<&mut Box<R>> where R: Sized + 'static {
    //     self.get_parent_data_ref()
    //         .and_then(|parent_data| parent_data.downcast_mut())
    // }
}

