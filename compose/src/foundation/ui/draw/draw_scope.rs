use auto_delegate::delegate;

use crate::foundation::geometry::{Density, Offset, Size};
use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::ui::draw::DrawContext;
use crate::foundation::ui::graphics::color::Color;

#[delegate]
pub trait DrawScope<'a> {
    fn get_draw_context(&self) -> &DrawContext<'a>;
    fn get_draw_context_mut(&mut self) -> &mut DrawContext<'a>;
    fn get_layout_direction(&self) -> LayoutDirection;

    fn get_density(&self) -> Density {
        self.get_draw_context().get_density()
    }

    fn get_size(&self) -> Size<f32> {
        self.get_draw_context().get_size()
    }

    fn get_center(&self) -> Offset<f32> {
        self.get_size().center()
    }

    fn draw_rect(&mut self, color: Color, top_left: Offset<f32>, size: Option<Size<f32>>, alpha: f32);
}