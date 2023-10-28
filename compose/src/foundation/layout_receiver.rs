use crate::foundation::geometry::Density;
use super::layout_direction::LayoutDirection;

pub struct LayoutReceiver {
    pub(crate) density: Density,
    pub(crate) layout_direction: LayoutDirection
}