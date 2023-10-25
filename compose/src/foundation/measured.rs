use auto_delegate::delegate;

#[delegate]
pub trait  Measured {
    fn get_measured_width(&self) -> usize;
    fn get_measured_height(&self) -> usize;
}

#[derive(Debug)]
pub(crate) struct MeasuredImpl {
    pub(crate) measured_width: usize,
    pub(crate) measured_height: usize,
}