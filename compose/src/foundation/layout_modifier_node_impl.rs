use std::fmt::{Debug, Formatter};
use auto_delegate::Delegate;
use crate::foundation::constraint::Constraint;
use crate::foundation::delegatable_node::DelegatableNode;
use crate::foundation::layout_modifier_node::LayoutModifierNode;
use crate::foundation::measurable::Measurable;
use crate::foundation::measure_scope::MeasureScope;
use crate::foundation::modifier::{NodeImpl, NodeKind, NodeKindPatch};

#[derive(Debug, Delegate)]
pub(crate) struct LayoutModifierNodeImpl {
    layout_modifier_node: Box<dyn LayoutModifierNode>,
    #[to(Node)]
    node_impl: NodeImpl,
}

impl LayoutModifierNodeImpl {
    pub(crate) fn new(layout_modifier_node: Box<dyn LayoutModifierNode>) -> Self {
        Self {
            layout_modifier_node,
            ..Default::default()
        }
    }
}

impl DelegatableNode for LayoutModifierNodeImpl {}

impl LayoutModifierNode for LayoutModifierNodeImpl {
    fn measure(&mut self, layout_receiver: &mut dyn MeasureScope, measurable: &dyn Measurable, constraint: &Constraint) {
        self.layout_modifier_node.measure(layout_receiver, measurable, constraint)
    }
}

impl NodeKindPatch for LayoutModifierNodeImpl {
    fn get_node_kind(&mut self) -> NodeKind {
        self.layout_modifier_node.get_node_kind()
    }
}