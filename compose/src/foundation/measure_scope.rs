use auto_delegate::delegate;

use crate::foundation::geometry::{Density, IntSize};
use crate::foundation::measure_result::MeasureResult;
use crate::foundation::placement_scope::PlacementScope;
use crate::foundation::utils::box_wrapper::WrapWithBox;

use super::layout_direction::LayoutDirection;

pub(crate) fn empty_place_action() -> Box<dyn FnOnce(&dyn PlacementScope)> {
    (|_: &dyn PlacementScope| {}).wrap_with_box()
}

#[delegate]
pub trait MeasureScope {
    fn get_density(&self) -> Density;
    fn get_layout_direction(&self) -> LayoutDirection;

    fn layout(
        &self,
        size: IntSize,
        place_action: Box<dyn FnOnce(&dyn PlacementScope)>,
    ) -> MeasureResult;
}

#[derive(Debug, Default)]
pub struct MeasureScopeImpl {
    pub(crate) density: Density,
    pub(crate) layout_direction: LayoutDirection,
}
