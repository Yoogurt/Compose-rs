use auto_delegate::delegate;

use super::{geometry::IntOffset, measure_scope::MeasureScope};
use super::placeable::Placeable;

#[delegate]
pub trait LookaheadCapablePlaceable: Placeable + MeasureScope {
    fn set_position(&mut self, position: IntOffset);
    fn get_position(&self) -> IntOffset;
}
