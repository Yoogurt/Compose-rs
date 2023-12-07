use skia_safe::Rect;
use crate::foundation::canvas::CanvasExtension;
use crate::foundation::geometry::{IntSize, Offset};
use crate::foundation::node::OwnedLayer;
use crate::foundation::utils::box_wrapper::WrapWithBox;
use crate::foundation::canvas::Canvas;
use crate::foundation::ui::graphics::graphics_layer_modifier::GraphicsLayerScope;

pub(crate) struct SkiaOwnedLayer {
    property: GraphicsLayerScope,
    draw_block: Box<dyn Fn(&mut dyn Canvas)>,
}

impl SkiaOwnedLayer {
    pub(crate) fn new(draw_block: impl Fn(&mut dyn Canvas) + 'static) -> Self {
        SkiaOwnedLayer {
            property: GraphicsLayerScope::new(),
            draw_block: draw_block.wrap_with_box(),
        }
    }

    fn apply_canvas_property(&self, canvas: &mut dyn Canvas) {
        canvas.translate(self.property.get_translation_x(), self.property.get_translation_y());
        canvas.scale(self.property.get_scale_x(), self.property.get_scale_y());
        (self.draw_block)(canvas);
    }
}

impl OwnedLayer for SkiaOwnedLayer {
    fn update_layer_property(&mut self, graphics_layer_scope: &GraphicsLayerScope) {
        self.property = graphics_layer_scope.clone();
    }

    fn is_in_layer(&self, position: Offset<f32>) {
        todo!()
    }

    fn draw_layer(&self, canvas: &mut dyn Canvas) {
        let alpha = self.property.get_alpha();
        if alpha >= 1.0f32 {
            canvas.with_save_layer(|canvas| {
                self.apply_canvas_property(canvas);
            });
        } else {
            canvas.with_save_layer_alpha(None, alpha, |canvas| {
                self.apply_canvas_property(canvas);
            });
        }
    }

    fn move_to(&mut self, position: Offset<f32>) {}

    fn resize(&mut self, size: IntSize) {}
}