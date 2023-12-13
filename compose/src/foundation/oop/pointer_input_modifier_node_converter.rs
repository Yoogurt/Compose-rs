use crate::foundation::modifier_node::PointerInputModifierNode;

pub(crate) trait PointerInputModifierNodeConverter {
    fn as_pointer_input_modifier_node(&self) -> Option<&dyn PointerInputModifierNode> {
        None
    }

    fn as_pointer_input_modifier_node_mut(&mut self) -> Option<&mut dyn PointerInputModifierNode> {
        None
    }
}