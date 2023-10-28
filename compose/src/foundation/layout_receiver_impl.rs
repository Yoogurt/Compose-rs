use crate::foundation::geometry::Density;
use crate::foundation::layout_result::{PlacementScope, PlacementScopeImpl};
use super::{layout_receiver::LayoutReceiver, layout_direction::LayoutDirection, measure_result::MeasureResult};

impl LayoutReceiver {
    pub(crate) fn new() -> LayoutReceiver {
        LayoutReceiver {
            density: Density::new(1.0, 1.0),
            layout_direction: LayoutDirection::Ltr,
        }
    }

    pub fn layout(&self, width: usize, height: usize, place_action: impl FnOnce(&dyn PlacementScope)) -> MeasureResult {
        let  place_scope = PlacementScopeImpl::new(width, height, self);
        place_action(&place_scope);
        (width, height).into()
    }
}