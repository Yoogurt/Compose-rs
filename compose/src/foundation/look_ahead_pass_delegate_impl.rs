use auto_delegate::MacroMarker;
use crate::foundation::{LookaheadPassDelegate, PlaceAction, MeasureResult, Constraint, Measured, Placeable, PlaceableImpl};
use crate::foundation::geometry::{IntSize, IntOffset};

impl LookaheadPassDelegate {
    pub(crate) fn new() -> Self {
        LookaheadPassDelegate {
            placeable_impl: PlaceableImpl::new()
        }
    }
}