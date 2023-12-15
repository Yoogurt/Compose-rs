use crate::foundation::geometry::Dp;

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
}