use std::any::Any;

use crate::foundation::utils::box_wrapper::WrapWithBox;

pub(crate) trait OptionThen<T> {
    fn then(self, f: impl FnOnce(T));
}

impl<T> OptionThen<T> for Option<T> {
    fn then(self, f: impl FnOnce(T)) {
        match self {
            Some(value) => f(value),
            None => {}
        }
    }
}

pub(crate) trait OptionalInstanceConverter {
    fn cast_or<R>(self, init: impl FnOnce() -> R) -> Box<R> where R: Sized + 'static;
}

impl OptionalInstanceConverter for Option<Box<dyn Any>> {
    fn cast_or<R>(self, init: impl FnOnce() -> R) -> Box<R> where R: Sized + 'static {
        match self {
            None => {
                init().wrap_with_box()
            }
            Some(instance) => {
                instance.downcast::<R>().unwrap_or_else(|_| {
                    init().wrap_with_box()
                })
            }
        }
    }
}