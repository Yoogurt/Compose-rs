use auto_delegate::delegate;
use crate::foundation::layout_modifier_node::LayoutModifierNode;

#[delegate]
pub trait LayoutNodeModifierConverter {
    fn as_layout_node_modifier(&self) -> Option<&dyn LayoutModifierNode> {
        None
    }
    fn as_layout_node_modifier_mut(&mut self) -> Option<&mut dyn LayoutModifierNode> {
        None
    }
}