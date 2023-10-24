use crate::foundation::MeasureResult;

 impl From<(usize, usize)> for MeasureResult {
    fn from(value: (usize, usize)) -> Self {
        MeasureResult {
            width: value.0,
            height: value.1,
        }
    }
}

impl From<MeasureResult> for (usize, usize) {
    fn from(value: MeasureResult) -> Self {
        (value.width,value.height)
    }
}