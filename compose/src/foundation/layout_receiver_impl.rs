use crate::foundation::{LayoutDirection, LayoutReceiver, MeasureResult};

impl LayoutReceiver {
    pub(crate) fn new() -> LayoutReceiver {
        LayoutReceiver {
            density: 1.0,
            font_scale: 1.0,
            layout_direction: LayoutDirection::Ltr,
        }
    }

    pub fn layout(&self, width: usize, height: usize) -> MeasureResult {
        (width, height).into()
    }
}