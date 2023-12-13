use crate::foundation::geometry::IntSize;
use crate::foundation::layout::layout_coordinates::LayoutCoordinates;

pub(crate) trait LayoutAwareModifierNode {
    fn on_placed(&self, coordinate: &dyn LayoutCoordinates) {}
    fn on_remeasured(&self, size: IntSize) {}
}