use crate::foundation::MeasureResult;

impl From<(usize, usize)> for MeasureResult {
    fn from(value: (usize, usize)) -> Self {
        MeasureResult {
            width: value.0,
            height: value.1,
        }
    }
}