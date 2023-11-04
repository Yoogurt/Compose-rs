use crate::foundation::constraint::Constraints;
use crate::foundation::delegatable_node::DelegatableNode;
use crate::foundation::layout_modifier_node::LayoutModifierNode;
use crate::foundation::measurable::Measurable;
use crate::foundation::measure_result::MeasureResult;
use crate::foundation::measure_scope::MeasureScope;
use crate::foundation::modifier::{NodeImpl, NodeKind, NodeKindPatch};
use crate::foundation::oop::any_converter::AnyConverter;
use auto_delegate::Delegate;
use std::any::Any;
use std::fmt::Debug;

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
            node_impl: NodeImpl::default(),
        }
    }
}

impl AnyConverter for LayoutModifierNodeImpl {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl DelegatableNode for LayoutModifierNodeImpl {}

impl LayoutModifierNode for LayoutModifierNodeImpl {
    fn measure(
        &mut self,
        measure_scope: &mut dyn MeasureScope,
        measurable: &mut dyn Measurable,
        constraint: &Constraints,
    ) -> MeasureResult {
        self.layout_modifier_node
            .measure(measure_scope, measurable, constraint)
    }
}

impl NodeKindPatch for LayoutModifierNodeImpl {
    fn get_node_kind(&mut self) -> NodeKind {
        self.layout_modifier_node.get_node_kind()
    }
}
