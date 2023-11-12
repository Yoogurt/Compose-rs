use crate::foundation::canvas::Canvas;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use auto_delegate::Delegate;
use crate::foundation::geometry::{Offset, Size};
use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::modifier::{ModifierNode, NodeKind, ModifierNodeExtension, ModifierElement};
use crate::foundation::modifier_node::DrawModifierNode;
use crate::foundation::node_coordinator::{DrawableNodeCoordinator, NodeCoordinator, NodeCoordinatorTrait, PerformDrawTrait};
use crate::foundation::ui::draw::{CanvasDrawScope, ContentDrawScope, DrawContext, DrawScope};
use crate::foundation::ui::graphics::color::Color;

#[derive(Delegate)]
pub(crate) struct LayoutNodeDrawScope<'a> {
    canvas_draw_scope: CanvasDrawScope<'a>,
    draw_node: Option<Rc<RefCell<dyn ModifierNode>>>,
}

impl<'a> LayoutNodeDrawScope<'a> {
    pub(crate) fn new(canvas_draw_scope: CanvasDrawScope<'a>) -> Self {
        Self {
            canvas_draw_scope: canvas_draw_scope,
            draw_node: None,
        }
    }

    pub(crate) fn get_draw_node(&mut self) -> Rc<RefCell<dyn ModifierNode>> {
        self.draw_node.clone().unwrap()
    }

    pub(crate) fn draw(mut self: Box<Self>, draw_node: Rc<RefCell<dyn ModifierNode>>) {
        draw_node.borrow().dispatch_for_kind(NodeKind::Draw, |element| {
            self.draw_node = Some(draw_node.clone());
            element.as_draw_modifier_node().unwrap().draw(self.deref_mut());
            self.draw_node = None;
        });
    }

    pub(crate) fn draw_into_canvas(&mut self, block: impl FnOnce(&mut dyn Canvas)) {
        block(self.canvas_draw_scope.get_draw_context_mut().get_canvas())
    }
}

impl<'a> DrawScope<'a> for LayoutNodeDrawScope<'a> {
    fn get_draw_context(&self) -> &DrawContext<'a> {
        self.canvas_draw_scope.get_draw_context()
    }

    fn get_draw_context_mut(&mut self) -> &mut DrawContext<'a> {
        self.canvas_draw_scope.get_draw_context_mut()
    }

    fn get_layout_direction(&self) -> LayoutDirection {
        todo!()
    }

    fn draw_rect(&mut self, color: Color, top_left: Offset<f32>, size: Option<Size<f32>>, alpha: f32) {
        self.canvas_draw_scope.draw_rect(color, top_left, size, alpha)
    }
}

impl<'a> ContentDrawScope<'a> for LayoutNodeDrawScope<'a> {
    fn draw_content(&mut self) {
        let draw_node = self.draw_node.clone().unwrap();

        self.draw_into_canvas(|canvas| {
            let next_draw_node = draw_node.borrow().next_draw_node();

            match next_draw_node {
                Some(next_draw_node) => {
                    next_draw_node.borrow().dispatch_for_kind(NodeKind::Draw, |it| {
                        perform_draw(it.as_draw_modifier_node().unwrap(), canvas)
                    });
                }
                None => {
                    let next_coordinator = {
                        let coordinator = draw_node.borrow().require_coordinator(NodeKind::Draw);
                        let coordinator_ref = coordinator.borrow();
                        if coordinator_ref.get_tail().as_ptr() as *const () == draw_node.as_ptr() as *const () {
                            coordinator_ref.get_wrapped().unwrap()
                        } else {
                            drop(coordinator_ref);
                            coordinator
                        }
                    };

                    next_coordinator.borrow().perform_draw(canvas);
                }
            }
        })
    }
}

fn perform_draw(draw_modifier_node: &dyn DrawModifierNode, canvas: &mut dyn Canvas) {
}