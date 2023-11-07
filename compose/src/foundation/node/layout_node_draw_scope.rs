use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use auto_delegate::Delegate;
use crate::foundation::geometry::{Offset, Size};
use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::modifier::{ModifierNode, DispatchForKind, NodeKind};
use crate::foundation::ui::draw::{CanvasDrawScope, ContentDrawScope, DrawContext, DrawScope};
use crate::foundation::ui::graphics::color::Color;

#[derive(Delegate)]
pub(crate) struct LayoutNodeDrawScope<'a> {
    canvas_draw_scope: Rc<RefCell<CanvasDrawScope<'a>>>,
    draw_node: Option<Rc<RefCell<dyn ModifierNode>>>
}

impl<'a> LayoutNodeDrawScope<'a> {
    pub(crate) fn new(canvas_draw_scope: CanvasDrawScope<'a>) -> Self {
        Self {
            canvas_draw_scope: Rc::new(RefCell::new(canvas_draw_scope)),
            draw_node: None,
        }
    }

    pub(crate) fn get_draw_node(&mut self) -> Rc<RefCell<dyn ModifierNode>> {
        self.draw_node.clone().unwrap()
    }

    pub(crate) fn draw(mut self: Box<Self>, draw_node: Rc<RefCell<dyn ModifierNode>>) {
        draw_node.dispatch_for_kind(NodeKind::DrawModifierNode, |draw| {
            self.draw_node = Some(draw_node.clone());

            self.canvas_draw_scope.clone().borrow_mut().draw(draw, |node, canvas_draw_scope| {
                node.as_draw_modifier_node().unwrap().draw(self.deref_mut())
            });

            self.draw_node = None;
        });
    }
}

impl<'a> DrawScope<'a> for LayoutNodeDrawScope<'a> {
    fn get_draw_context(&self) -> &DrawContext<'a> {
        // self.canvas_draw_scope.borrow().get_draw_context()
        todo!()
    }

    fn get_layout_direction(&self) -> LayoutDirection {
        // self.canvas_draw_scope.borrow().get_layout_direction()
        todo!()
    }

    fn draw_rect(&mut self, color: Color, top_left: Offset<f32>, size: Option<Size<f32>>, alpha: f32) {
        // self.canvas_draw_scope.borrow_mut().draw_rect(color, top_left, size, alpha)
        todo!()
    }
}

impl<'a> ContentDrawScope<'a> for LayoutNodeDrawScope<'a> {
    fn draw_content(&mut self) {

    }
}