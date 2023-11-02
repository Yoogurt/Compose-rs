use crate::foundation::geometry::Density;
use crate::foundation::placement_scope::PlacementScope;
use crate::foundation::placement_scope_impl::PlacementScopeImpl;
use super::{measure_scope::MeasureScopeImpl, measure_scope::MeasureScope, layout_direction::LayoutDirection, measure_result::MeasureResult};

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

    fn layout(&self, width: usize, height: usize,  place_action: &mut dyn FnMut(&dyn PlacementScope)) -> MeasureResult {
        let place_scope = PlacementScopeImpl::new(width, height, self);
        place_action(&place_scope);
        (width, height).into()
    }
}