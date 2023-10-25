
use auto_delegate::{delegate, Delegate};
use super::{layout_direction::LayoutDirection, constraint::Constraint, measured::{Measured, MeasuredImpl}, geometry::{IntSize, IntOffset}, measure_result::MeasureResult};

pub trait PlacementScope {
    fn parent_width(&self) -> usize;
    fn parent_layout_direction(&self) -> LayoutDirection;
}

pub type PlaceAction = Box<dyn FnOnce(&dyn PlacementScope)>;
pub type MeasureAction = Box<dyn FnOnce() -> MeasureResult>;

#[delegate]
pub trait Placeable: Measured {
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;

    fn set_measured_size(&mut self, size: IntSize);
    fn get_measured_size(&self) -> IntSize;

    fn set_measurement_constraint(&mut self, constraint: &Constraint);
    fn get_measurement_constraint(&self) -> &Constraint;

    fn perfroming_measure(&mut self, constraint: &Constraint, block: MeasureAction) -> &dyn Placeable;

    fn place_at(&mut self, position: IntOffset, z_index: f32, place_action: PlaceAction);
}

#[derive(Debug, Delegate)]
pub(crate) struct PlaceableImpl {
    pub(crate) width: usize,
    pub(crate) height: usize,
    #[to(Measured)]
    pub(crate) measured: MeasuredImpl,
    pub(crate) measured_size: IntSize,
    pub(crate) measurement_constraint: Constraint,
}