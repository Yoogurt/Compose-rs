use crate::foundation::constraint::Constraints;
use crate::foundation::delegatable_node::DelegatableNode;
use crate::foundation::measurable::Measurable;
use crate::foundation::measure_result::MeasureResult;
use crate::foundation::measure_scope::MeasureScope;
use crate::foundation::modifier::NodeKindPatch;
use crate::foundation::oop::modifier_node_converter::LayoutNodeModifierConverter;

pub trait LayoutModifierNode: DelegatableNode + LayoutNodeModifierConverter + NodeKindPatch {
    fn measure(
        &mut self,
        measure_scope: &mut dyn MeasureScope,
        measurable: &mut dyn Measurable,
        constraint: &Constraints,
    ) -> MeasureResult;
}
