use std::borrow::Borrow;

use compose_macro::Composable;

use crate::foundation::composer::Composer;
use crate::foundation::snapshot_value::SnapShotValue;

#[Composable]
pub fn remember<R, T>(key: &R, calculation: impl FnOnce() -> T) -> SnapShotValue<T> where T: 'static, R: Sized + PartialEq<R> + 'static {
    Composer::cache(key, calculation)
}