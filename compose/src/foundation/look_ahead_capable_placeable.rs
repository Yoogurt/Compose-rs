use super::placeable::Placeable;
use super::{geometry::IntOffset, measure_scope::MeasureScope};

pub(crate) trait LookaheadCapablePlaceable: Placeable + MeasureScope {
    fn set_position(&mut self, position: IntOffset);
    fn get_position(&self) -> IntOffset;
}
