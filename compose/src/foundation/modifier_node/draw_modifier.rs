use crate::foundation::delegatable_node::DelegatableNode;
use crate::foundation::ui::draw::ContentDrawScope;

pub(crate) trait DrawModifierNode: DelegatableNode {
    fn draw(&self, draw_scope: &mut dyn ContentDrawScope);
    fn on_measure_result_changed(&mut self) {}
}