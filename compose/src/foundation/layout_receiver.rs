use super::layout_direction::LayoutDirection;

pub struct LayoutReceiver {
    pub(crate) density: f32,
    pub(crate) font_scale:f32,
    pub(crate) layout_direction: LayoutDirection
}