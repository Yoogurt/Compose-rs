use crate::foundation::constraint::Constraints;
use crate::foundation::geometry::{CoerceIn, IntOffset, IntSize};
use crate::foundation::measured::MeasuredImpl;
use crate::foundation::placeable::Placeable;
use crate::foundation::placeable_place_at::PlaceablePlaceAt;
use auto_delegate::Delegate;

#[derive(Debug, Delegate, Default)]
pub(crate) struct PlaceableImpl {
    pub(crate) size: IntSize,
    #[to(Measured)]
    pub(crate) measured: MeasuredImpl,
    pub(crate) measured_size: IntSize,
    pub(crate) measurement_constraint: Constraints,


}

impl PlaceableImpl {
    pub(crate) fn new() -> Self {
        PlaceableImpl {
            size: IntSize::zero(),
            measured: MeasuredImpl::new(),
            measured_size: IntSize::zero(),
            measurement_constraint: Constraints::unbounded(),
        }
    }
}

impl PlaceableImpl {
    fn recalculate_width_and_height(&mut self) {
        *self.size.width_mut() = self
            .measured_size
            .width()
            .coerce_in(self.measurement_constraint.width_range());
        *self.size.height_mut() = self
            .measured_size
            .height()
            .coerce_in(self.measurement_constraint.height_range());
    }
}

impl PlaceablePlaceAt for PlaceableImpl {
    fn place_at(&mut self, _position: IntOffset, _z_index: f32) {
        unimplemented!("place_at to PlaceableImpl should implement by yourself");
    }
}

impl Placeable for PlaceableImpl {
    fn get_size(&self) -> IntSize {
        self.size
    }

    fn set_measured_size(&mut self, size: IntSize) {
        self.measured_size = size;
    }

    fn get_measured_size(&self) -> IntSize {
        self.measured_size
    }

    fn set_measurement_constraint(&mut self, constraint: &Constraints) {
        self.measurement_constraint = *constraint;
    }

    fn get_measurement_constraint(&self) -> Constraints {
        self.measurement_constraint
    }
}
