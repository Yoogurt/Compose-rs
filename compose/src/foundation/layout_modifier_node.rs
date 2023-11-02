use crate::foundation::constraint::Constraint;
use crate::foundation::delegatable_node::DelegatableNode;
use crate::foundation::layout_receiver::MeasureScope;
use crate::foundation::measurable::Measurable;

pub trait LayoutModifierNode : DelegatableNode {
    fn measure(&mut self, layout_receiver: &mut dyn MeasureScope, measurable: &dyn Measurable, constraint: &Constraint);
}