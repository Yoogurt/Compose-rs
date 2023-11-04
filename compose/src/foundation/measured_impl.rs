use super::measured::{MeasuredImpl, Measured};
use std::any::Any;

impl MeasuredImpl {
    pub(crate) fn new() -> MeasuredImpl {
        MeasuredImpl {
            measured_width: 0,
            measured_height: 0,
        }
    }
}

impl Measured for MeasuredImpl {
    fn get_measured_width(&self) -> usize {
        self.measured_width
    }

    fn get_measured_height(&self) -> usize {
        self.measured_height
    }
}
