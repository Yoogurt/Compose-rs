use auto_delegate::delegate;

use crate::foundation::modifier_node::LayoutModifierNode;

#[delegate]
pub trait LayoutModifierNodeConverter {
    fn as_layout_modifier_node(&self) -> Option<&dyn LayoutModifierNode> {
        None
    }
    fn as_layout_modifier_node_mut(&mut self) -> Option<&mut dyn LayoutModifierNode> { None }
}