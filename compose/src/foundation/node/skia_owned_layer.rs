use std::ops::Mul;
use skia_safe::{Matrix, Rect};
use crate::foundation::canvas::CanvasExtension;
use crate::foundation::geometry::{IntSize, Offset};
use crate::foundation::node::OwnedLayer;
use crate::foundation::utils::box_wrapper::WrapWithBox;
use crate::foundation::canvas::Canvas;
use crate::foundation::geometry::skia_extension::SkiaPointExtension;
use crate::foundation::ui::graphics::graphics_layer_modifier::GraphicsLayerScope;

pub(crate) struct SkiaOwnedLayer {
    property: GraphicsLayerScope,
    draw_block: Box<dyn Fn(&mut dyn Canvas)>,
    matrix: Matrix,
}

impl SkiaOwnedLayer {
    pub(crate) fn new(draw_block: impl Fn(&mut dyn Canvas) + 'static) -> Self {
        SkiaOwnedLayer {
            property: GraphicsLayerScope::new(),
            draw_block: draw_block.wrap_with_box(),
            matrix: Matrix::default(),
        }
    }

    fn apply_canvas_property(&self, canvas: &mut dyn Canvas) {
        canvas.translate(self.property.get_translation_x(), self.property.get_translation_y());
        canvas.scale(self.property.get_scale_x(), self.property.get_scale_y());
        (self.draw_block)(canvas);
    }

    fn update_matrix(&mut self) {
        let matrix = &mut self.matrix;
        matrix.reset();

        {
            let mut rotated_matrix = Matrix::default();
            rotated_matrix.set_scale((self.property.get_scale_x(), self.property.get_scale_y()), None);
            *matrix = *matrix * rotated_matrix;
        }

        {
            let mut translated_matrix = Matrix::default();
            translated_matrix.set_translate((self.property.get_translation_x(), self.property.get_translation_y()));
            *matrix = *matrix * translated_matrix;
        }
    }
}

impl OwnedLayer for SkiaOwnedLayer {
    fn update_layer_property(&mut self, graphics_layer_scope: &GraphicsLayerScope) {
        self.property = graphics_layer_scope.clone();
        self.update_matrix();
    }

    fn is_in_layer(&self, position: Offset<f32>) -> bool {
        if !self.property.get_clip() {
            return true;
        }

        let x = position.x;
        let y = position.y;

        let size = self.property.get_size();
        0f32 <= x && x < size.width && 0f32 <= y && y < size.height
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

    fn map_offset(&self, point: Offset<f32>, inverse: bool) -> Offset<f32> {
        if inverse {
            todo!()
        } else {
            self.matrix
        }.map_point(point).into()
    }
}