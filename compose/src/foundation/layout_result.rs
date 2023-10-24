use std::ops::DerefMut;
use crate::foundation::geometry::IntSize;

pub trait PlacementScope {
    fn parent_width(&self) -> usize;
    fn parent_layout_direction(&self) -> LayoutDirection;
}

pub type PlaceAction = &'static dyn FnOnce(&dyn PlacementScope);

pub trait Placeable: Measured {
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;

    fn set_measured_size(&mut self, size: IntSize);
    fn get_measured_size(&self) -> IntSize;

    fn get_measurement_constraint(&self) -> &Constraint;

    fn place_at(&mut self, position: IntOffset, z_index: f32, place_action: PlaceAction);
}

#[derive(Debug)]
pub struct PlaceableImpl {
    width: usize,
    height: usize,
    measured: Box<dyn Measured>,
    measured_size: IntSize,
    measurement_constraint: Constraint,
}