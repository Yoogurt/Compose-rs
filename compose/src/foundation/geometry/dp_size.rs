use crate::foundation::geometry::{Density, Dp, Size};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DpSize {
    pub width: Dp,
    pub height: Dp,
}

impl DpSize {
    pub const ZERO: DpSize = DpSize {
        width: Dp::ZERO,
        height: Dp::ZERO,
    };

    pub const fn new(width: Dp, height: Dp) -> Self {
        Self {
            width,
            height,
        }
    }

    pub fn to_size(&self, density: Density) -> Size<f32> {
        Size::<f32>::new(self.width.to_px(density), self.height.to_px(density))
    }
}