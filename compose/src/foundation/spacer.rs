use crate::foundation::modifier::Modifier;
use compose_macro::Composable;
use crate as compose;
use crate::foundation::constraint::Constraints;
use crate::foundation::measurable::Measurable;
use crate::foundation::measure_scope::{empty_place_action, MeasureScope, MeasureScopeLayoutAction};
use crate::foundation::utils::box_wrapper::WrapWithBox;
use crate::widgets::layout::Layout;
use crate::foundation::measure_result::MeasureResult;

fn spacer_measure_policy(measure_scope: &dyn MeasureScope, measurables: &mut [&mut dyn Measurable], constraints: &Constraints) -> MeasureResult {
    let width = if constraints.has_fixed_width() { constraints.max_width } else { 0 };
    let height = if constraints.has_fixed_height() { constraints.max_height } else { 0 };

    measure_scope.layout_without_place((width, height))
}

#[Composable]
pub fn Spacer(modifier: Modifier) {
    Layout(modifier, spacer_measure_policy.wrap_with_box(), || {})
}