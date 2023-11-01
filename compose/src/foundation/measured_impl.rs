use super::measured::{MeasuredImpl, Measured};
use std::any::Any;

impl MeasuredImpl {
    pub(crate) fn new() -> MeasuredImpl {
        MeasuredImpl {
            measured_width: 0,
            measured_height: 0,
            parent_data: None,
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

    fn set_parent_data(&mut self, parent_data: Option<Box<dyn Any>>) {
        self.parent_data = parent_data;
    }

    fn get_parent_data(&self) -> Option<&Box<dyn Any>> {
        self.parent_data.as_ref()
    }

    fn get_parent_data_mut(&mut self) -> Option<&mut Box<dyn Any>> {
        self.parent_data.as_mut()
    }
}
