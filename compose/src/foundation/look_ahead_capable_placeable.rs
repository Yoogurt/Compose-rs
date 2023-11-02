use super::{measure_scope::MeasureScope, geometry::IntOffset};
use super::{placeable::Placeable};

pub(crate) trait LookaheadCapablePlaceable: Placeable + MeasureScope {
    fn set_position(&mut self, position: IntOffset);
    fn get_position(&self) -> IntOffset;
}
