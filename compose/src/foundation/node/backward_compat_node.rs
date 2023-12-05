use std::cell::RefCell;
use std::rc::Rc;

use auto_delegate::Delegate;
use compose_foundation_macro::ModifierElement;

use crate::foundation::constraint::Constraints;
use crate::foundation::delegatable_node::DelegatableNode;
use crate::foundation::measurable::Measurable;
use crate::foundation::measure_result::MeasureResult;
use crate::foundation::measure_scope::MeasureScope;
use crate::foundation::modifier::{ModifierElement, ModifierNode, ModifierNodeImpl, NodeKind, NodeKindPatch};
use crate::foundation::modifier_node::{DrawModifierNode, LayoutModifierNode};
use crate::foundation::node_coordinator::DrawableNodeCoordinator;
use crate::foundation::oop::LayoutModifierNodeConverter;
use crate::foundation::ui::draw::ContentDrawScope;

#[derive(Debug, Delegate, ModifierElement)]
#[Impl(Layout)]
pub(crate) struct BackwardsCompatNode {
    element: Rc<RefCell<dyn ModifierElement>>,

    #[to(ModifierNode, DelegatableNode)]
    modifier_node_impl: ModifierNodeImpl,
}

impl BackwardsCompatNode {
    pub(crate) fn new(element: Rc<RefCell<dyn ModifierElement>>) -> Self {
        BackwardsCompatNode {
            element,
            modifier_node_impl: ModifierNodeImpl::default(),
        }
    }
}

impl NodeKindPatch for BackwardsCompatNode {
    fn get_node_kind(&self) -> NodeKind {
        self.element.borrow_mut().get_node_kind()
    }
}

impl LayoutModifierNode for BackwardsCompatNode {
    fn measure(&self, measure_scope: &mut dyn MeasureScope, measurable: &mut dyn Measurable, constraint: &Constraints) -> MeasureResult {
        self.element.borrow_mut().as_layout_modifier_node_mut().unwrap().measure(measure_scope, measurable, constraint)
    }
}

impl DrawModifierNode for BackwardsCompatNode {
    fn draw(&self, draw_scope: &mut dyn ContentDrawScope) {
        self.element.borrow_mut().as_draw_modifier_node_mut().unwrap().draw(draw_scope)
    }

    fn on_measure_result_changed(&mut self) {}
}