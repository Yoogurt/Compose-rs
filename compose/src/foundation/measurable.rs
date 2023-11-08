use std::cell::RefCell;
use std::rc::Rc;
use super::{
    constraint::Constraints, measure_result::MeasureResult, measure_scope::MeasureScope,
    placeable::Placeable,
};
use crate::foundation::intrinsic_measurable::IntrinsicMeasurable;

pub trait Measurable: IntrinsicMeasurable {
    fn measure(&mut self, constraint: &Constraints) -> Rc<RefCell<dyn Placeable>>;

    fn as_placeable(&mut self) -> Rc<RefCell<dyn Placeable>>;
    fn as_measurable_mut(&mut self) -> &mut dyn Measurable;
}

pub type SingleChildMeasurePolicy =
    Box<dyn FnMut(&mut dyn MeasureScope, &mut dyn Measurable, &Constraints) -> MeasureResult>;

pub type SingleChildMeasurePolicyUnBox =
    fn(&mut dyn MeasureScope, &mut dyn Measurable, &Constraints) -> MeasureResult;

pub type MultiChildrenMeasurePolicy = Box<
    dyn FnMut(& dyn MeasureScope, &mut [&mut dyn Measurable], &Constraints) -> MeasureResult,
>;

pub type MultiChildrenMeasurePolicyUnBox =
    fn(& dyn MeasureScope, &mut [&mut dyn Measurable], &Constraints) -> MeasureResult;
