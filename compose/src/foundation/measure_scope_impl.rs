use super::{
    layout_direction::LayoutDirection, measure_result::MeasureResult, measure_scope::MeasureScope,
    measure_scope::MeasureScopeImpl,
};
use crate::foundation::geometry::{Density, IntSize};
use crate::foundation::placement_scope::PlacementScope;
use crate::foundation::placement_scope_impl::PlacementScopeImpl;

impl MeasureScopeImpl {
    pub(crate) fn new() -> Self {
        MeasureScopeImpl {
            density: Density::new(1.0, 1.0),
            layout_direction: LayoutDirection::Ltr,
        }
    }
}

impl MeasureScope for MeasureScopeImpl {
    fn get_density(&self) -> Density {
        self.density
    }

    fn get_layout_direction(&self) -> LayoutDirection {
        self.layout_direction
    }

    fn layout(
        &self,
        size: IntSize,
        place_action: &mut dyn FnMut(&dyn PlacementScope),
    ) -> MeasureResult {
        let place_scope = PlacementScopeImpl::new(size.width(), size.height(), self);
        // place_action(&place_scope);
        size.into()
    }
}
