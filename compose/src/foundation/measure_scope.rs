use auto_delegate::delegate;

use crate::foundation::geometry::{Density, IntSize};
use crate::foundation::measure_result::MeasureResult;
use crate::foundation::placement_scope::PlacementScope;
use crate::foundation::utils::box_wrapper::WrapWithBox;

use super::layout_direction::LayoutDirection;

pub(crate) fn empty_place_action(_: &dyn PlacementScope) {}

#[delegate]
pub trait MeasureScope {
    fn get_density(&self) -> Density;
    fn get_layout_direction(&self) -> LayoutDirection;
}

#[derive(Debug, Default)]
pub struct MeasureScopeImpl {
    pub(crate) density: Density,
    pub(crate) layout_direction: LayoutDirection,
}

pub trait MeasureScopeLayoutAction {
    fn layout(
        &self,
        size: impl Into<IntSize>,
        place_action: impl FnOnce(&dyn PlacementScope) + 'static,
    ) -> MeasureResult;

    fn layout_without_place(
        &self,
        size: impl Into<IntSize>,
    ) -> MeasureResult;
}

impl<T> MeasureScopeLayoutAction for T where T: ?Sized + MeasureScope {
    fn layout(&self, size: impl Into<IntSize>, place_action: impl FnOnce(&dyn PlacementScope) + 'static) -> MeasureResult {
        MeasureResult::new(size.into(), Some(place_action.wrap_with_box()))
    }

    fn layout_without_place(
        &self,
        size: impl Into<IntSize>,
    ) -> MeasureResult {
        MeasureResult::new(size.into(), None)
    }
}