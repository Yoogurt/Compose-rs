use crate::foundation::modifier_node::DrawModifierNode;

pub(crate) trait DrawModifierNodeConverter {
    fn as_draw_modifier_node(&self) -> Option<&dyn DrawModifierNode> {
        None
    }
    fn as_draw_modifier_node_mut(&mut self) -> Option<&mut dyn DrawModifierNode> {
        None
    }
}