pub trait Measurable {
    fn measure(&mut self, constraint: &Constraint) -> &mut dyn Placeable;
}

pub type SingleChildMeasurePolicy = fn(layout_receiver: LayoutReceiver, measurable: &mut dyn Measurable, constraint: &Constraint)
                                       -> MeasureResult;

pub type MultiChildrenMeasurePolicy = fn(layout_receiver: LayoutReceiver, measurables: &mut [&mut dyn Measurable], constraint: &Constraint)
                                         -> MeasureResult;