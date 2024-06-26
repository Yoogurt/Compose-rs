use std::fmt::Debug;

use auto_delegate::delegate;

use crate::foundation::placeable_place_at::PlaceablePlaceAt;

use super::{
    constraint::Constraints, geometry::IntSize, measure_result::MeasureResult, measured::Measured,
};

pub type MeasureAction = Box<dyn FnOnce() -> MeasureResult>;

#[delegate]
pub trait Placeable: Measured + PlaceablePlaceAt + Debug {
    fn get_size(&self) -> IntSize;

    fn set_measured_size(&mut self, size: IntSize);
    fn get_measured_size(&self) -> IntSize;

    fn set_measurement_constraint(&mut self, constraint: &Constraints);
    fn get_measurement_constraint(&self) -> Constraints;
}
