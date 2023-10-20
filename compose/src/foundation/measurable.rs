pub trait Measurable {
    fn measure(&mut self, constraint: &Constraint) -> Placeable;
}

pub type MeasurePolicyDelegate = fn(measurable: &[&mut dyn Measurable], constraint: &Constraint)
                                    -> MeasureResult;