
use auto_delegate::delegate;
use crate::foundation::geometry::IntSize;

pub trait PlacementScope {
    fn parent_width(&self) -> usize;
    fn parent_layout_direction(&self) -> LayoutDirection;
}

pub type PlaceAction = &'static dyn FnOnce(&dyn PlacementScope);

#[delegate]
pub trait Placeable: Measured {
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;

    fn set_measured_size(&mut self, size: IntSize);
    fn get_measured_size(&self) -> IntSize;

    fn set_measurement_constraint(&mut self, constraint: &Constraint);
    fn get_measurement_constraint(&self) -> &Constraint;

    fn perfroming_measure(&mut self, constraint: &Constraint, block: & mut dyn FnMut() -> MeasureResult) -> &dyn Placeable;

    fn place_at(&mut self, position: IntOffset, z_index: f32, place_action: PlaceAction);
}

#[derive(Debug, Delegate)]
pub(crate) struct PlaceableImpl {
    width: usize,
    height: usize,
    #[to(Measured)]
    measured: MeasuredImpl,
    measured_size: IntSize,
    measurement_constraint: Constraint,
}