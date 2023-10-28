use crate::foundation::geometry::{IntOffset, IntSize, CoerceIn};
use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::layout_receiver::LayoutReceiver;
use crate::foundation::layout_result::PlacementScopeImpl;

use super::{layout_result::{PlaceableImpl, Placeable, PlacementScope}, measured::MeasuredImpl, constraint::Constraint};

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

    fn place_at(&mut self, _position: IntOffset, _z_index: f32) {}

    fn set_measurement_constraint(&mut self, constraint: &Constraint) {
        self.measurement_constraint = *constraint;
    }

    fn get_measurement_constraint(&self) -> &Constraint {
        &self.measurement_constraint
    }

    // fn perfroming_measure(&mut self, constraint: &Constraint, block: MeasureAction) -> &dyn Placeable {
    //     self.set_measurement_constraint(constraint);
    //     self.set_measured_size(block().into());
    //     return self as &dyn Placeable;
    // }
}

impl PlacementScope for PlacementScopeImpl<'_> {
    fn parent_width(&self) -> usize {
        self.width
    }

    fn parent_height(&self) -> usize {
        self.height
    }

    fn parent_layout_direction(&self) -> LayoutDirection {
        self.scope.layout_direction
    }

    fn place_relative(&self, placeable: &mut dyn Placeable, x: i32, y: i32) {
        self.place_relative_with_z(placeable, x, y, 0.0)
    }

    fn place_relative_with_z(&self, placeable: &mut dyn Placeable, x: i32, y: i32, z_index: f32) {
        // mirror
        if self.parent_layout_direction() == LayoutDirection::Ltr || self.parent_width() == 0 {
            placeable.place_at((x, y).into(), z_index)
        } else {
            placeable.place_at((self.parent_width() as i32 - placeable.get_width() as i32 - x, y).into(), z_index)
        }
    }
}

impl<'a> PlacementScopeImpl<'a> {
    pub(crate) fn new(width: usize, height: usize, layout_receiver: &'a LayoutReceiver) -> Self {
        PlacementScopeImpl {
            width,
            height,
            scope: layout_receiver,
        }
    }
}