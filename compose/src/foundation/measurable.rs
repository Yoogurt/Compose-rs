
use super::{constraint::Constraint, layout_result::Placeable, layout_receiver::LayoutReceiver, measure_result::MeasureResult};

pub trait Measurable {
    fn measure(&mut self, constraint: &Constraint) -> &mut dyn Placeable;
}

pub type SingleChildMeasurePolicy = Box<dyn FnMut(LayoutReceiver, &mut dyn Measurable, &Constraint)
                                       -> MeasureResult>;

pub type MultiChildrenMeasurePolicy = Box<dyn FnMut(LayoutReceiver, &mut [&mut dyn Measurable], &Constraint)
                                         -> MeasureResult>;