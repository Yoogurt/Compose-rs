use crate::foundation::constraint::Constraint;
use crate::foundation::delegatable_node::DelegatableNode;
use crate::foundation::layout_receiver::LayoutReceiver;
use crate::foundation::measurable::Measurable;

pub trait LayoutModifierNode : DelegatableNode {
    fn measure(&mut self, layout_receiver: LayoutReceiver, measurable: &dyn Measurable, constraint: &Constraint);
}