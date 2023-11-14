use crate as compose;
use compose_macro::Composable;
use core::any::Any;
use std::borrow::Borrow;
use crate::foundation::composer::Composer;
use crate::foundation::snapshot_value::SnapShotValue;

pub fn remember< R, T>(key: &R, calculation: impl FnOnce() -> T) -> SnapShotValue<T> where R: Sized + PartialEq<R> + 'static {
    Composer::cache(key, calculation)
}