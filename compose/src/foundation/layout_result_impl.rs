use std::ops::{Deref, DerefMut};
use crate::foundation::{Constraint, Measured, MeasuredImpl, Placeable, PlaceableImpl, PlacementScope};
use crate::foundation::geometry::{IntOffset, IntSize, CoerceIn};

impl Measured for PlaceableImpl {
    fn get_measured_width(&self) -> usize {
        self.measured.get_measured_width()
    }

    fn get_measured_height(&self) -> usize {
        self.measured.get_measured_height()
    }
}

impl PlaceableImpl {
    pub(crate) fn new() -> Self {
        PlaceableImpl {
            width: 0,
            height: 0,
            measured: Box::new(MeasuredImpl::new()),
            measured_size: IntSize::zero(),
            measurement_constraint: Constraint::unbounded()
        }
    }
}

impl PlaceableImpl {
    fn recalculate_width_and_height(&mut self) {
        self.width = self.measured_size.width().coerce_in(self.measurement_constraint.width_range());
        self.height = self.measured_size.height().coerce_in(self.measurement_constraint.height_range());
    }
}

impl Placeable for PlaceableImpl {
    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn get_measured_size(&self) -> IntSize {
        self.measured_size
    }

    fn set_measured_size(&mut self, size: IntSize) {
        self.measured_size = size;
    }

    fn place_at(&mut self, position: IntOffset, z_index: f32, place_action: &dyn FnOnce(&dyn PlacementScope)) {}

    fn get_measurement_constraint(&self) -> &Constraint {
        &self.measurement_constraint
    }
}