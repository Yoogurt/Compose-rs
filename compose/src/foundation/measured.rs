pub trait  Measured : Debug{
    fn get_measured_width(&self) -> usize;
    fn get_measured_height(&self) -> usize;
}

#[derive(Debug)]
pub(crate) struct MeasuredImpl {
    measured_width: usize,
    measured_height: usize,
}