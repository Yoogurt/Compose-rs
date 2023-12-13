use crate::foundation::modifier_node::ParentDataModifierNode;

pub(crate) trait ParentDataModifierNodeConverter {
    fn as_parent_data_modifier_node(&self) -> Option<&dyn ParentDataModifierNode> {
        None
    }
    fn as_parent_data_modifier_node_mut(&mut self) -> Option<&mut dyn ParentDataModifierNode> {
        None
    }
}