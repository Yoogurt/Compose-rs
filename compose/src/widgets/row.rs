use compose_macro::Composable;

use crate as compose;
use crate::foundation::geometry::Density;
use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::modifier::{Modifier, modifier_node_element_creator, modifier_node_element_updater};
use crate::foundation::ui::align::{Alignment, AlignmentVertical};
use crate::foundation::ui::arrangement::ArrangementHorizontal;
use crate::foundation::ui::size_mode::SizeMode;
use crate::widgets::cross_axis_alignment::CrossAxisAlignment;
use crate::widgets::layout::Layout;
use crate::widgets::row_column::{row_column_measure_policy, RowColumnWeightScope};
use crate::widgets::row_column_measurement_helper::LayoutOrientation;
use crate::widgets::row_column_measurement_helper::VerticalAlignModifier;

impl Modifier {
    pub fn vertical_align(self, row_scope: &dyn RowScope, alignment_vertical: AlignmentVertical) -> Modifier {
        row_scope.vertical_align(self, alignment_vertical)
    }
}

pub trait RowScope: RowColumnWeightScope {
    fn vertical_align(&self, modifier: Modifier, alignment_vertical: AlignmentVertical) -> Modifier;
}

struct RowScopeImpl;

const INSTANCE: &dyn RowScope = &RowScopeImpl {};

fn vertical_align_modifier(alignment_vertical: AlignmentVertical) -> Modifier {
    Modifier::ModifierNodeElement {
        create: modifier_node_element_creator(move || {
            VerticalAlignModifier::new(alignment_vertical)
        }),
        update: modifier_node_element_updater(move |vertical_align_modifier: &mut VerticalAlignModifier| {
            vertical_align_modifier.alignment_vertical = alignment_vertical;
        }),
    }
}

impl RowColumnWeightScope for RowScopeImpl {}

impl RowScope for RowScopeImpl {
    fn vertical_align(&self, modifier: Modifier, alignment_vertical: AlignmentVertical) -> Modifier {
        modifier.then(vertical_align_modifier(alignment_vertical))
    }
}

pub struct RowParams {
    pub vertical_alignment: AlignmentVertical,
    pub horizontal_arrangement: ArrangementHorizontal,
}

impl Default for RowParams {
    fn default() -> Self {
        Self {
            vertical_alignment: Alignment::TOP,
            horizontal_arrangement: ArrangementHorizontal::START,
        }
    }
}

fn row_arrangement(total_size: usize, size: &[usize], layout_direction: LayoutDirection, arrangement: &mut [i32], density: Density) -> Vec<i32> {
    ArrangementHorizontal::START.arrange(density, total_size, size, layout_direction)
}

#[Composable]
pub fn Row(modifier: Modifier, params: RowParams,
           mut content: impl FnMut(&dyn RowScope)) {
    Layout(modifier, row_column_measure_policy(LayoutOrientation::Horizontal,
                                               row_arrangement,
                                               params.horizontal_arrangement.spacing(),
                                               SizeMode::Wrap,
                                               CrossAxisAlignment::VERTICAL(params.vertical_alignment),
    ), || {
        content(INSTANCE);
    });
}