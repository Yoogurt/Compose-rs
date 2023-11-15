use std::fmt::Alignment;
use crate::foundation::measurable::Measurable;
use compose_macro::Composable;
use crate as compose;
use crate::foundation::constraint::Constraints;
use crate::foundation::measurable::MultiChildrenMeasurePolicy;
use crate::foundation::modifier::Modifier;
use crate::foundation::utils::box_wrapper::WrapWithBox;
use crate::widgets::layout::Layout;
use crate::foundation::measure_scope::MeasureScope;
use crate::foundation::ui::align::AlignmentHorizontal;
use crate::foundation::ui::arrangement::ArrangementHorizontal;

pub trait ColumnScope {}

const INSTANCE: &dyn ColumnScope = &ColumnScopeImpl {};

struct ColumnScopeImpl {}

impl ColumnScope for ColumnScopeImpl {}

fn column_measure_policy() -> MultiChildrenMeasurePolicy {
    (|measure_scope: &dyn MeasureScope, measurables: &mut [&mut dyn Measurable], constraints: &Constraints| {
        todo!()
    }).wrap_with_box()
}

pub struct ColumnParams {
    modifier: Modifier,
    vertical_arrangement: ArrangementHorizontal,
    horizontal_alignment: AlignmentHorizontal,
}

#[Composable]
fn Column(params: ColumnParams,
          mut content: impl FnMut(&dyn ColumnScope)) {
    Layout(params.modifier, column_measure_policy(), || {
        content(INSTANCE);
    });
}