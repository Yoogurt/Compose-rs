use crate::foundation::canvas::Canvas;
use crate::foundation::ui::draw::ContentDrawScope;

pub trait DrawModifierNode {
    fn draw(&self, draw_scope: Box<dyn ContentDrawScope>);
}