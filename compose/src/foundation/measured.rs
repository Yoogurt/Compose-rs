use auto_delegate::delegate;
use std::any::Any;

#[delegate]
pub trait Measured {
    fn get_measured_width(&self) -> usize;
    fn get_measured_height(&self) -> usize;
}

#[derive(Debug, Default)]
pub(crate) struct MeasuredImpl {
    pub(crate) measured_width: usize,
    pub(crate) measured_height: usize,
    // pub(crate) parent_data: Option<Box<dyn Any>>
}
