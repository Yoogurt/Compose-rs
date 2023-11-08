use super::layout_direction::LayoutDirection;
use crate::foundation::geometry::{Density, IntSize};
use crate::foundation::measure_result::MeasureResult;
use crate::foundation::placement_scope::PlacementScope;
use auto_delegate::delegate;

#[delegate]
pub trait MeasureScope {
    fn get_density(&self) -> Density;
    fn get_layout_direction(&self) -> LayoutDirection;

    fn layout(
        &self,
        size: IntSize,
        place_action: &mut dyn FnMut(&dyn PlacementScope),
    ) -> MeasureResult;
}

#[derive(Debug, Default)]
pub struct MeasureScopeImpl {
    pub(crate) density: Density,
    pub(crate) layout_direction: LayoutDirection,
}
