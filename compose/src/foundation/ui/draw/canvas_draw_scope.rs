use skia_safe::{Point, Rect};

use crate::foundation::canvas::Canvas;
use crate::foundation::geometry::Offset;
use crate::foundation::geometry::Size;
use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::ui::draw::{DrawContext, DrawScope};
use crate::foundation::ui::graphics::color::Color;

pub(crate) struct CanvasDrawScope<'a> {
    draw_context: DrawContext<'a>,
    layout_direction: LayoutDirection,
}

impl<'a> DrawScope<'a> for CanvasDrawScope<'a> {
    fn get_layout_direction(&self) -> LayoutDirection {
        self.layout_direction
    }

    fn get_draw_context(&self) -> &DrawContext<'a> {
        &self.draw_context
    }

    fn get_draw_context_mut(&mut self) -> &mut DrawContext<'a> {
        &mut self.draw_context
    }

    fn draw_rect(&mut self, color: Color, top_left: Offset<f32>, size: Option<Size<f32>>, alpha: f32) {
        let size = match size {
            Some(size) => size,
            None => self.draw_context.get_size(),
        };

        let canvas = self.draw_context.get_canvas();

        let layer = canvas.save_layer();
        canvas.draw_circle(Point::new(200.0,200.0), 100.0, Color::GREEN);
        drop(layer);
        canvas.draw_rect(color, Rect::new(top_left.x, top_left.y, size.width, size.height));
    }
}

impl<'a> CanvasDrawScope<'a> {
    pub(crate) fn new(draw_context: DrawContext<'a>, layout_direction: LayoutDirection) -> Self {
        Self {
            draw_context,
            layout_direction,
        }
    }

    pub(crate) fn draw<T>(&mut self, params: T, mut block: impl FnOnce(T, &mut Self)) {
        block(params, self)
    }
}
