use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Weak;

#[repr(transparent)]
#[derive(Debug)]
pub(crate) struct OptionalWeak<T> {
    weak: Option<Weak<RefCell<T>>>
}

impl<T> Default for OptionalWeak<T> {
    fn default() -> Self {
        Self {
            weak: None
        }
    }
}

impl<T> Deref for OptionalWeak<T> {
    type Target = Option<Weak<RefCell<T>>>;

    fn deref(&self) -> &Self::Target {
        &self.weak
    }
}

impl<T> DerefMut for OptionalWeak<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.weak
    }
}

impl<T> From<Option<Weak<RefCell<T>>>> for OptionalWeak<T> {
    fn from(weak: Option<Weak<RefCell<T>>>) -> Self {
        Self {
            weak
        }
    }
}