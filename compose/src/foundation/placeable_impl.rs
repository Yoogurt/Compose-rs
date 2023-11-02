use auto_delegate::Delegate;
use crate::foundation::constraint::Constraint;
use crate::foundation::geometry::{CoerceIn, IntOffset, IntSize};
use crate::foundation::measured::MeasuredImpl;
use crate::foundation::placeable::Placeable;
use crate::foundation::placeable_place_at::PlaceablePlaceAt;

#[derive(Debug, Delegate, Default)]
pub(crate) struct PlaceableImpl {
    pub(crate) width: usize,
    pub(crate) height: usize,
    #[to(Measured)]
    pub(crate) measured: MeasuredImpl,
    pub(crate) measured_size: IntSize,
    pub(crate) measurement_constraint: Constraint,
}

impl PlaceableImpl {
    pub(crate) fn new() -> Self {
        PlaceableImpl {
            width: 0,
            height: 0,
            measured: MeasuredImpl::new(),
            measured_size: IntSize::zero(),
            measurement_constraint: Constraint::unbounded(),
        }
    }
}

impl PlaceableImpl {
    fn recalculate_width_and_height(&mut self) {
        self.width = self.measured_size.width().coerce_in(self.measurement_constraint.width_range());
        self.height = self.measured_size.height().coerce_in(self.measurement_constraint.height_range());
    }
}

impl PlaceablePlaceAt for PlaceableImpl {
    fn place_at(&mut self, _position: IntOffset, _z_index: f32) {
        unimplemented!("place_at to PlaceableImpl should implement by yourself");
    }
}

impl Placeable for PlaceableImpl {
    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn set_measured_size(&mut self, size: IntSize) {
        self.measured_size = size;
    }

    fn get_measured_size(&self) -> IntSize {
        self.measured_size
    }

    fn set_measurement_constraint(&mut self, constraint: &Constraint) { self.measurement_constraint = *constraint; }

    fn get_measurement_constraint(&self) -> &Constraint {
        &self.measurement_constraint
    }
}