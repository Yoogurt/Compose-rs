use super::{Density, Dp};

impl Density {
    pub fn new(density: f32, font_scale: f32) -> Self {
        Self {
            density,
            font_scale
        }
    }

    pub fn dp_to_px(&self, dp: Dp) -> f32 {
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