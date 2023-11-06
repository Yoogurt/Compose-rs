use auto_delegate::Delegate;
use crate::foundation::ui::draw::{CanvasDrawScope, ContentDrawScope, DrawContext, DrawScope};

#[derive(Delegate)]
pub(crate) struct LayoutNodeDrawScope<'a> {
    #[to(DrawScope)]
    cavas_draw_scope: CanvasDrawScope<'a>
}

impl<'a> LayoutNodeDrawScope<'a> {
    pub(crate) fn new(canvas_draw_scope: CanvasDrawScope<'a>) -> Self {
        Self {
            cavas_draw_scope: canvas_draw_scope
        }
    }
}

impl<'a> ContentDrawScope<'a> for LayoutNodeDrawScope<'a> {
    fn draw_content(self: Box<Self>) {

    }
}