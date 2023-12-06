use crate::foundation::ui::graphics::graphics_layer_modifier::GraphicsLayerScope;
use auto_delegate::delegate;
use crate::foundation::geometry::IntOffset;
use std::rc::Rc;

#[delegate]
pub trait PlaceablePlaceAt {
    fn place_at(&mut self, position: IntOffset, z_index: f32, layer_block: Option<Rc<dyn Fn(&mut GraphicsLayerScope)>>);
}
