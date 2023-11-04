use super::{
    constraint::Constraints, geometry::IntSize, measure_result::MeasureResult, measured::Measured,
};
use crate::foundation::placeable_place_at::PlaceablePlaceAt;
use auto_delegate::delegate;

pub type MeasureAction = Box<dyn FnOnce() -> MeasureResult>;

#[delegate]
pub trait Placeable: Measured + PlaceablePlaceAt {
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;

    fn set_measured_size(&mut self, size: IntSize);
    fn get_measured_size(&self) -> IntSize;

    fn set_measurement_constraint(&mut self, constraint: &Constraints);
    fn get_measurement_constraint(&self) -> &Constraints;
}
