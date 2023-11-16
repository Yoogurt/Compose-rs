use crate::widgets::row_column_measurement_helper::{HorizontalAlignModifier, LayoutOrientation, RowColumnParentData};
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use crate::foundation::geometry::Dp;
use crate::foundation::ui::arrangement::{Arrangement, ArrangementVerticalTrait};
use crate::foundation::measurable::Measurable;
use compose_macro::Composable;
use crate as compose;
use crate::foundation::constraint::Constraints;
use crate::foundation::geometry::Density;
use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::measurable::MultiChildrenMeasurePolicy;
use crate::foundation::modifier::{Modifier, ModifierNode};
use crate::foundation::utils::box_wrapper::WrapWithBox;
use crate::widgets::layout::Layout;
use crate::foundation::measure_scope::MeasureScope;
use crate::foundation::placeable::Placeable;
use crate::foundation::placement_scope::PlacementScope;
use crate::foundation::ui::align::AlignmentHorizontal;
use crate::foundation::ui::align::AlignmentStruct;
use crate::foundation::ui::size_mode::SizeMode;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;
use crate::widgets::cross_axis_alignment::CrossAxisAlignment;
use crate::widgets::row_column_measurement_helper::RowColumnMeasureHelper;

pub trait ColumnScope {
    fn weight(&self, modifier: Modifier, weight: f32) -> Modifier;
    fn align(&self, modifier: Modifier, alignment_horizontal: AlignmentHorizontal) -> Modifier;
}

const INSTANCE: &dyn ColumnScope = &ColumnScopeImpl {};

struct ColumnScopeImpl {}

// fn row_column_parent_data(weight: f32, fill: bool) -> Modifier {
//     Modifier::ModifierNodeElement {
//         create: (move || {
//             let mut parent_data = RowColumnParentData::new();
//             {
//                 let parent_data = parent_data.borrow_mut();
//                 parent_data.weight = weight;
//                 parent_data.fill = fill;
//             }
//             parent_data as Rc<RefCell<dyn ModifierNode>>
//         }).wrap_with_box(),
//         update: (move |mut modifier_node: RefMut<dyn ModifierNode>| {
//             if let Some(row_column_parent_data) = modifier_node.as_any_mut().downcast_mut::<RowColumnParentData>() {
//                 row_column_parent_data.weight = weight;
//                 row_column_parent_data.fill = fill;
//             }
//         }).wrap_with_box(),
//     }
// }

fn horizontal_align_modifier(alignment_horizontal: AlignmentHorizontal) -> Modifier {
    Modifier::ModifierNodeElement {
        create: (move || {
            HorizontalAlignModifier::new(alignment_horizontal) as Rc<RefCell<dyn ModifierNode>>
        }).wrap_with_box(),
        update: (move |mut modifier_node: RefMut<dyn ModifierNode>| {
            if let Some(horizontal_align_modifier) = modifier_node.as_any_mut().downcast_mut::<HorizontalAlignModifier>() {
                horizontal_align_modifier.alignment_horizontal = alignment_horizontal;
            }
        }).wrap_with_box(),
    }
}

impl ColumnScope for ColumnScopeImpl {
    fn weight(&self, modifier: Modifier, weight: f32) -> Modifier {
        // modifier.then(row_column_parent_data(weight, false))
        todo!()
    }

    fn align(&self, modifier: Modifier, alignment_horizontal: AlignmentHorizontal) -> Modifier {
        modifier.then(horizontal_align_modifier(alignment_horizontal))
    }
}

fn row_column_measure_policy(
    layout_orientation: LayoutOrientation,
    arrangement: fn(total_size: usize, size: &[usize], layout_direction: LayoutDirection, arrangement: &mut [i32], density: Density) -> Vec<i32>,
    arrangement_spacing: Dp,
    cross_axis_size: SizeMode,
    cross_axis_alignment: CrossAxisAlignment,
) -> MultiChildrenMeasurePolicy {
    (move |measure_scope: &dyn MeasureScope, measurables: &mut [&mut dyn Measurable], constraints: &Constraints| {
        let mut placeables: Vec<Option<Rc<RefCell<dyn Placeable>>>> = vec![None; measurables.len()];

        let row_column_measurement_helper = RowColumnMeasureHelper {
            orientation: layout_orientation,
            arrangement,
            arrangement_spacing,
            cross_axis_size,
            cross_axis_alignment,
        };

        let measure_result = row_column_measurement_helper.measure_without_placing(measure_scope, measurables, &mut placeables, constraints, 0..=measurables.len());

        let layout_width;
        let layout_height;

        match layout_orientation {
            LayoutOrientation::Horizontal => {
                layout_width = measure_result.main_axis_size;
                layout_height = measure_result.cross_axis_size;
            }
            LayoutOrientation::Vertical => {
                layout_width = measure_result.cross_axis_size;
                layout_height = measure_result.main_axis_size;
            }
        }

        measure_scope.layout((layout_width, layout_height).into(), (move |placement_scope: &dyn PlacementScope| {
            todo!()
        }).wrap_with_box())
    }).wrap_with_box()
}

pub struct ColumnParams {
    modifier: Modifier,
    vertical_arrangement: &'static dyn ArrangementVerticalTrait,
    horizontal_alignment: &'static AlignmentHorizontal,
}

impl Default for ColumnParams {
    fn default() -> Self {
        Self {
            modifier: Modifier,
            vertical_arrangement: Arrangement::TOP,
            horizontal_alignment: &AlignmentStruct::START,
        }
    }
}

#[Composable]
fn Column(params: ColumnParams,
          mut content: impl FnMut(&dyn ColumnScope)) {
    // Layout(params.modifier, column_measure_policy(), || {
    //     content(INSTANCE);
    // });
}