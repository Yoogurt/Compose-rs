use crate::foundation::intrinsic_measurable::IntrinsicMeasurable;
use super::{constraint::Constraints, placeable::Placeable, measure_scope::MeasureScope, measure_result::MeasureResult};

pub trait Measurable: IntrinsicMeasurable {
    fn measure(&mut self, constraint: &Constraints) -> &mut dyn Placeable;

    fn as_placeable_mut(&mut self) -> &mut dyn Placeable;
    fn as_measurable_mut(&mut self) -> &mut dyn Measurable;
}

pub type SingleChildMeasurePolicy = Box<dyn FnMut(&mut dyn MeasureScope, &mut dyn Measurable, &Constraints)
    -> MeasureResult>;

pub type SingleChildMeasurePolicyUnBox = fn(&mut dyn MeasureScope, &mut dyn Measurable, &Constraints)
                                            -> MeasureResult;

pub type MultiChildrenMeasurePolicy = Box<dyn FnMut(&mut dyn MeasureScope, &mut [&mut dyn Measurable], &Constraints)
    -> MeasureResult>;

pub type MultiChildrenMeasurePolicyUnBox = fn(&mut dyn MeasureScope, &mut [&mut dyn Measurable], &Constraints)
                                              -> MeasureResult;