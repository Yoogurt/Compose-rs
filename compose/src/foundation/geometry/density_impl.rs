use std::default;

use super::{Density, Dp};

impl Density {
    pub fn new(density: f64, font_scale: f64) -> Self {
        Self {
            density,
            font_scale,
        }
    }

    pub fn dp_to_px(&self, dp: Dp) -> f64 {
        dp.value * self.density
    }

    pub fn dp_round_to_px(&self, dp: Dp) -> usize {
        let px = self.dp_to_px(dp);

        if px.is_infinite() {
            usize::MAX
        } else {
            px.round() as usize
        }
    }
}

impl Default for Density {
    fn default() -> Self {
        Self {
            density: 1f64,
            font_scale: 1f64,
        }
    }
}
