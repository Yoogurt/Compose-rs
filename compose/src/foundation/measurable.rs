pub trait Measurable {
    fn measure(&mut self, constraint: &Constraint) -> Placeable;
}

pub type MeasurePolicyDelegate = fn(layout_receiver: LayoutReceiver, measurable: &mut [&mut dyn Measurable], constraint: &Constraint)
                                    -> MeasureResult;