use crate::foundation::canvas::Canvas;
use crate::foundation::geometry::{IntSize, Offset};
use crate::foundation::ui::graphics::graphics_layer_modifier::GraphicsLayerScope;

pub trait OwnedLayer {
    fn update_layer_property(&mut self, graphics_layer_scope: &GraphicsLayerScope);
    fn is_in_layer(&self, position: Offset<f32>) -> bool;
    fn draw_layer(&self, canvas: &mut dyn Canvas);

    fn resize(&mut self, size: IntSize);
    fn move_to(&mut self, position: Offset<f32>);

    fn map_offset(&self, point: Offset<f32>, inverse: bool) -> Offset<f32>;
}