use super::{layout_receiver::MeasureScope, geometry::IntOffset};
use crate::foundation::placeable::Placeable;

pub(crate) trait LookaheadCapablePlaceable: Placeable + MeasureScope {
    fn set_position(&mut self, position: IntOffset);
    fn get_position(&self) -> IntOffset;
}
