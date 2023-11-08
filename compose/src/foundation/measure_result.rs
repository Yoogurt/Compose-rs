use auto_delegate::delegate;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct MeasureResult {
    pub(crate) width: usize,
    pub(crate) height: usize,
}

#[delegate]
pub trait MeasureResultProvider {
    fn set_measured_result(&mut self, measure_result: MeasureResult);

    fn get_measured_result(&self) -> MeasureResult;
}

impl<T> From<T> for MeasureResult where T: Into<(usize, usize)>{
    fn from(value: T) -> Self {
        let dimension = value.into();

        MeasureResult {
            width: dimension.0,
            height: dimension.1,
        }
    }
}
