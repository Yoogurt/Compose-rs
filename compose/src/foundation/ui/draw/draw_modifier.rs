use crate::foundation::canvas::Canvas;
use crate::foundation::delegatable_node::DelegatableNode;
use crate::foundation::ui::draw::ContentDrawScope;

pub trait DrawModifierNode : DelegatableNode {
    fn draw(&self, draw_scope: &mut dyn ContentDrawScope);
}