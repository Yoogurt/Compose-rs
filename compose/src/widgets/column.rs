use compose_macro::Composable;

use crate as compose;
use crate::foundation::geometry::Density;
use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::modifier::{Modifier, ModifierNodeElement};
use crate::foundation::ui::align::AlignmentHorizontal;
use crate::foundation::ui::align::AlignmentStruct;
use crate::foundation::ui::arrangement::ArrangementVertical;
use crate::foundation::ui::size_mode::SizeMode;
use crate::widgets::cross_axis_alignment::CrossAxisAlignment;
use crate::widgets::layout::Layout;
use crate::widgets::row_column::{row_column_measure_policy, RowColumnWeightScope};
use crate::widgets::row_column_measurement_helper::{HorizontalAlignModifier, LayoutOrientation};

impl Modifier {
    pub fn horizontal_align(self, column_scope: &dyn ColumnScope, alignment_horizontal: AlignmentHorizontal) -> Modifier {
        column_scope.horizontal_align(self, alignment_horizontal)
    }
}

pub trait ColumnScope: RowColumnWeightScope {
    fn horizontal_align(&self, modifier: Modifier, alignment_horizontal: AlignmentHorizontal) -> Modifier;
}

const INSTANCE: &dyn ColumnScope = &ColumnScopeImpl {};

struct ColumnScopeImpl {}

fn horizontal_align_modifier(alignment_horizontal: AlignmentHorizontal) -> Modifier {
    ModifierNodeElement(
        "HorizontalAlignElement",
        move || {
            HorizontalAlignModifier::new(alignment_horizontal)
        },
        move |horizontal_align_modifier: &mut HorizontalAlignModifier| {
            horizontal_align_modifier.alignment_horizontal = alignment_horizontal;
        },
    )
}

impl RowColumnWeightScope for ColumnScopeImpl {}

impl ColumnScope for ColumnScopeImpl {
    fn horizontal_align(&self, modifier: Modifier, alignment_horizontal: AlignmentHorizontal) -> Modifier {
        modifier.then(horizontal_align_modifier(alignment_horizontal))
    }
}

pub struct ColumnParams {
    pub vertical_arrangement: ArrangementVertical,
    pub horizontal_alignment: AlignmentHorizontal,
}

impl Default for ColumnParams {
    fn default() -> Self {
        Self {
            vertical_arrangement: ArrangementVertical::TOP,
            horizontal_alignment: AlignmentStruct::START,
        }
    }
}

fn column_arrangement(total_size: usize, size: &[usize], layout_direction: LayoutDirection, arrangement: &mut [i32], density: Density) -> Vec<i32> {
    ArrangementVertical::TOP.arrange(density, total_size, size, layout_direction)
}

#[Composable]
pub fn Column(modifier: Modifier, params: ColumnParams,
              mut content: impl FnMut(&dyn ColumnScope)) {
    Layout(modifier, row_column_measure_policy(LayoutOrientation::Vertical,
                                               column_arrangement,
                                               params.vertical_arrangement.spacing(),
                                               SizeMode::Wrap,
                                               CrossAxisAlignment::HORIZONTAL(params.horizontal_alignment),
    ), || {
        content(INSTANCE);
    });
}