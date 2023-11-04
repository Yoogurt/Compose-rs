use crate::foundation::geometry::IntOffset;
use crate::foundation::look_ahead_capable_placeable::LookaheadCapablePlaceable;
use crate::foundation::measure_result::MeasureResult;
use crate::foundation::measure_scope::MeasureScopeImpl;
use crate::foundation::placeable_impl::PlaceableImpl;
use crate::foundation::placeable_place_at::PlaceablePlaceAt;
use auto_delegate::Delegate;

#[derive(Default, Debug, Delegate)]
pub(crate) struct LookaheadCapablePlaceableImpl {
    #[to(Placeable, Measured)]
    placeable_impl: PlaceableImpl,
    #[to(MeasureScope)]
    measure_scope_impl: MeasureScopeImpl,
    position: IntOffset,
    measure_result: MeasureResult,
}

impl PlaceablePlaceAt for LookaheadCapablePlaceableImpl {
    fn place_at(&mut self, position: IntOffset, z_index: f32) {
        unimplemented!("unimplemented place_at for LookaheadCapablePlaceableImpl")
    }
}

impl LookaheadCapablePlaceable for LookaheadCapablePlaceableImpl {
    fn set_position(&mut self, position: IntOffset) {
        self.position = position;
    }

    fn get_position(&self) -> IntOffset {
        self.position
    }
}
