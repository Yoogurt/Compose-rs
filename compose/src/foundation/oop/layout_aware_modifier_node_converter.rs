use crate::foundation::modifier_node::{LayoutAwareModifierNode};

pub trait LayoutAwareModifierNodeConverter {
    fn as_layout_aware_modifier_node(&self) -> Option<&dyn LayoutAwareModifierNode> {
        None
    }
    fn as_layout_aware_modifier_node_mut(&mut self) -> Option<&mut dyn LayoutAwareModifierNode> {
        None
    }
}