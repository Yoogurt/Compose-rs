use crate::foundation::{LayoutNode, LayoutReceiver, MeasureResult};

impl LayoutReceiver {
    pub(crate) fn new() -> LayoutReceiver {
        LayoutReceiver {
        }
    }

    pub fn layout(&self, width: usize, height: usize) -> MeasureResult {
        (width, height).into()
    }
}