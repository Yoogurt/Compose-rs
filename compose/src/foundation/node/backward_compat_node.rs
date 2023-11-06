use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use auto_delegate::Delegate;
use crate::foundation::constraint::Constraints;
use crate::foundation::delegatable_node::DelegatableNode;
use crate::foundation::layout_modifier_node::LayoutModifierNode;
use crate::foundation::measurable::Measurable;
use crate::foundation::measure_result::MeasureResult;
use crate::foundation::measure_scope::MeasureScope;
use crate::foundation::modifier::{ModifierNode, ModifierNodeImpl, NodeKind, NodeKindPatch};
use crate::foundation::oop::layout_node_modifier_converter::LayoutNodeModifierConverter;
use crate::{impl_node_kind_any, implement_any_by_self};

#[derive(Debug, Delegate)]
pub(crate) struct BackwardsCompatNode {
    element: Rc<RefCell<dyn Any>>,

    #[to(ModifierNode)]
    modifier_node_impl: ModifierNodeImpl,
}

implement_any_by_self!(BackwardsCompatNode);
impl DelegatableNode for BackwardsCompatNode {}

impl LayoutNodeModifierConverter for BackwardsCompatNode {
    fn as_layout_node_modifier(&self) -> Option<&dyn LayoutModifierNode> {
        Some(self)
    }

    fn as_layout_node_modifier_mut(&mut self) -> Option<&mut dyn LayoutModifierNode> {
        Some(self)
    }
}

impl NodeKindPatch for BackwardsCompatNode {
    fn get_node_kind(&mut self) -> NodeKind {
        todo!()
    }
}

impl LayoutModifierNode for BackwardsCompatNode {
    fn measure(&mut self, measure_scope: &mut dyn MeasureScope, measurable: &mut dyn Measurable, constraint: &Constraints) -> MeasureResult {
        todo!()
    }
}