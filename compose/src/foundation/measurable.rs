
use super::{constraint::Constraint, placeable::Placeable, measure_scope::MeasureScope, measure_result::MeasureResult};

pub trait Measurable {
    fn measure(&mut self, constraint: &Constraint) -> &mut dyn Placeable;
}

pub type SingleChildMeasurePolicy = Box<dyn FnMut(&mut dyn MeasureScope, &mut dyn Measurable, &Constraint)
                                       -> MeasureResult>;

pub type SingleChildMeasurePolicyUnBox = fn(&mut dyn  MeasureScope, &mut dyn Measurable, &Constraint)
                                            -> MeasureResult;

pub type MultiChildrenMeasurePolicy = Box<dyn FnMut(&mut dyn MeasureScope, &mut [&mut dyn Measurable], &Constraint)
                                         -> MeasureResult>;

pub type MultiChildrenMeasurePolicyUnBox = fn(&mut dyn MeasureScope, &mut [&mut dyn Measurable], &Constraint)
                                              -> MeasureResult;