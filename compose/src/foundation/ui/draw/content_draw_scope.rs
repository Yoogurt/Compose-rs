use crate::foundation::ui::draw::DrawScope;

pub trait ContentDrawScope<'a>: DrawScope<'a> {
    fn draw_content(self: Box<Self>);
}