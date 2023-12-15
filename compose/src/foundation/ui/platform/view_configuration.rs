use crate::foundation::geometry::dp_size::DpSize;
use crate::foundation::geometry::IntoDp;

#[derive(Debug, Copy, Clone)]
pub(crate) struct ViewConfiguration {
    pub minimumTouchTargetSize: DpSize
}

impl Default for ViewConfiguration {
    fn default() -> Self {
        Self {
            minimumTouchTargetSize: DpSize::new(48.dp(), 48.dp())
        }
    }
}