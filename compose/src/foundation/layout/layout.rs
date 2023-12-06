use std::fmt::{Debug, Formatter};
use crate::foundation::measure_result::MeasureResult;
use auto_delegate::Delegate;
use compose_foundation_macro::ModifierElement;
use crate::foundation::constraint::Constraints;
use crate::foundation::measurable::{Measurable, SingleChildMeasurePolicy, SingleChildMeasurePolicyDelegate};
use crate::foundation::measure_scope::MeasureScope;
use crate::foundation::modifier::{Modifier, ModifierNodeElement, ModifierNodeImpl};
use crate::foundation::modifier::NodeKind::Layout;
use crate::foundation::modifier_node::LayoutModifierNode;
use crate::impl_node_kind_layout;

impl Modifier {
    pub fn layout(self, measure: impl FnMut(&mut dyn MeasureScope, &mut dyn Measurable, &Constraints) -> MeasureResult + 'static) -> Modifier {
        self.then(layout_element(SingleChildMeasurePolicyDelegate(measure)))
    }
}

#[derive(Delegate, ModifierElement)]
#[Impl(Layout)]
struct LayoutElement {
    measure: SingleChildMeasurePolicy,

    #[to(ModifierNode)]
    node_impl: ModifierNodeImpl,
}
impl_node_kind_layout!(LayoutElement);

impl Debug for LayoutElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LayoutElement").field("measure", &self.measure.as_ptr()).finish()
    }
}

impl LayoutModifierNode for LayoutElement {
    fn measure(&self, measure_scope: &mut dyn MeasureScope, measurable: &mut dyn Measurable, constraint: &Constraints) -> MeasureResult {
        (self.measure.borrow_mut())(measure_scope, measurable, constraint)
    }
}

fn layout_element(measure: SingleChildMeasurePolicy) -> Modifier {
    let measure_for_update = measure.clone();

    ModifierNodeElement(
        move || {
            LayoutElement {
                measure: measure.clone(),
                node_impl: Default::default(),
            }
        },
        move |node: &mut LayoutElement| {
            node.measure = measure_for_update.clone();
        },
    )
}