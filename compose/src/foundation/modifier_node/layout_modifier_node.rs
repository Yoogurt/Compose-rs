use crate::foundation::constraint::Constraints;
use crate::foundation::delegatable_node::DelegatableNode;
use crate::foundation::geometry::IntSize;
use crate::foundation::measurable::Measurable;
use crate::foundation::measure_result::MeasureResult;
use crate::foundation::measure_scope::MeasureScope;
use crate::foundation::oop::LayoutModifierNodeConverter;

pub trait LayoutModifierNode: DelegatableNode + LayoutModifierNodeConverter  {
    fn measure(
        & self,
        measure_scope: &mut dyn MeasureScope,
        measurable: &mut dyn Measurable,
        constraint: &Constraints,
    ) -> MeasureResult;
}