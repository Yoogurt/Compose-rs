use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use auto_delegate::Delegate;
use compose_foundation_macro::ModifierElement;
use crate::foundation::constraint::Constraints;
use crate::foundation::delegatable_node::DelegatableNode;
use crate::foundation::layout_modifier_node::LayoutModifierNode;
use crate::foundation::measurable::Measurable;
use crate::foundation::measure_result::MeasureResult;
use crate::foundation::measure_scope::MeasureScope;
use crate::foundation::modifier::{ModifierElement, ModifierNode, ModifierNodeImpl, NodeKind, NodeKindPatch};
use crate::foundation::oop::LayoutModifierNodeConverter;
use crate::{impl_node_kind_any};
use crate::foundation::ui::draw::{ContentDrawScope, DrawModifierNode};

#[derive(Debug, Delegate, ModifierElement)]
#[Impl(LayoutModifierNodeConverter)]
pub(crate) struct BackwardsCompatNode {
    element: Rc<RefCell<dyn ModifierElement>>,

    #[to(ModifierNode)]
    modifier_node_impl: ModifierNodeImpl,
}

impl BackwardsCompatNode {
    pub(crate) fn new(element: Rc<RefCell<dyn ModifierElement>>) -> Self {
        BackwardsCompatNode {
            element,
            modifier_node_impl: ModifierNodeImpl::default()
        }
    }
}

impl DelegatableNode for BackwardsCompatNode {}

impl NodeKindPatch for BackwardsCompatNode {
    fn get_node_kind(& self) -> NodeKind {
        self.element.borrow_mut().get_node_kind()
    }
}

impl LayoutModifierNode for BackwardsCompatNode {
    fn measure(&mut self, measure_scope: &mut dyn MeasureScope, measurable: &mut dyn Measurable, constraint: &Constraints) -> MeasureResult {
        self.element.borrow_mut().as_layout_modifier_node_mut().unwrap().measure(measure_scope, measurable, constraint)
    }
}

impl DrawModifierNode for BackwardsCompatNode {
    fn draw(& self, draw_scope: &dyn ContentDrawScope) {
        self.element.borrow_mut().as_draw_modifier_node_mut().unwrap().draw(draw_scope)
    }
}