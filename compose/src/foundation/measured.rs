use auto_delegate::delegate;
use std::any::Any;

#[delegate]
pub trait Measured {
    fn get_measured_width(&self) -> usize;
    fn get_measured_height(&self) -> usize;

    fn set_parent_data(&mut self, parent_data: Option<Box<dyn Any>>);
    fn get_parent_data(&self) -> Option<&Box<dyn Any>>;
    fn get_parent_data_mut(&mut self) -> Option<&mut Box<dyn Any>>;
}

#[derive(Debug, Default)]
pub(crate) struct MeasuredImpl {
    pub(crate) measured_width: usize,
    pub(crate) measured_height: usize,
    pub(crate) parent_data: Option<Box<dyn Any>>
}