use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use auto_delegate::Delegate;
use crate::foundation::modifier::{ModifierNode, DispatchForKind, NodeKind};
use crate::foundation::ui::draw::{CanvasDrawScope, ContentDrawScope, DrawContext, DrawScope};

#[derive(Delegate)]
pub(crate) struct LayoutNodeDrawScope<'a> {
    #[to(DrawScope)]
    cavas_draw_scope: CanvasDrawScope<'a>,
    draw_node: Option<Rc<RefCell<dyn ModifierNode>>>
}

impl<'a> LayoutNodeDrawScope<'a> {
    pub(crate) fn new(canvas_draw_scope: CanvasDrawScope<'a>) -> Self {
        Self {
            cavas_draw_scope: canvas_draw_scope,
            draw_node: None,
        }
    }

    pub(crate) fn get_draw_node(&mut self) -> Rc<RefCell<dyn ModifierNode>> {
        self.draw_node.clone().unwrap()
    }

    pub(crate) fn draw(mut self: Box<Self>, draw_node: Rc<RefCell<dyn ModifierNode>>) {
        draw_node.dispatch_for_kind(NodeKind::DrawModifierNode, |draw| {
            self.draw_node = Some(draw_node.clone());

            self.cavas_draw_scope.draw(|| {
                draw.as_draw_modifier_node_mut().unwrap().draw(self.deref());
            });

            self.draw_node = None;
        });
    }
}

impl<'a> ContentDrawScope<'a> for LayoutNodeDrawScope<'a> {
    fn draw_content(&mut self) {

    }
}