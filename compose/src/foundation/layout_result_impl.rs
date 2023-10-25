
use crate::foundation::geometry::{IntOffset, IntSize, CoerceIn};

use super::{layout_result::{PlaceableImpl, Placeable, PlacementScope}, measured::MeasuredImpl, constraint::Constraint, measure_result::MeasureResult};

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

    fn place_at(&mut self, _position: IntOffset, _z_index: f32, _place_action: &dyn FnOnce(&dyn PlacementScope)) {}

    fn set_measurement_constraint(&mut self, constraint: &Constraint) {
        self.measurement_constraint = *constraint;
    }

    fn get_measurement_constraint(&self) -> &Constraint {
        &self.measurement_constraint
    }

    fn perfroming_measure(&mut self, constraint: &Constraint, block: & mut dyn FnMut() -> MeasureResult) -> &dyn Placeable {
        self.set_measurement_constraint(constraint);
        self.set_measured_size(block().into());
        return self as &dyn Placeable;
    }
}