use crate as compose;
use compose_macro::Composable;
use core::any::Any;
use std::borrow::Borrow;
use crate::foundation::composer::Composer;

pub fn remember< T>(key: &dyn Any, calculation: impl FnOnce() -> T) -> T {
    Composer::cache(&[key], calculation)
}

// #[Composable]
// pub fn remember_data<T>(keys: Vec<&dyn Any>, calculation: impl FnOnce() -> T) -> T {
//
// }