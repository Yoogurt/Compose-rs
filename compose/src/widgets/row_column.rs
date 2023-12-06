use crate::foundation::measurable::MultiChildrenMeasurePolicyDelegate;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use crate::foundation::constraint::Constraints;
use crate::foundation::geometry::{Density, Dp};
use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::measurable::{Measurable, MultiChildrenMeasurePolicy};
use crate::foundation::measure_scope::{empty_place_action, MeasureScope, MeasureScopeLayoutAction};
use crate::foundation::modifier::{Modifier, ModifierNodeElement, ModifierNode};
use crate::foundation::placeable::Placeable;
use crate::foundation::placement_scope::PlacementScope;
use crate::foundation::ui::size_mode::SizeMode;
use crate::foundation::utils::box_wrapper::WrapWithBox;
use crate::widgets::cross_axis_alignment::CrossAxisAlignment;
use crate::widgets::row_column_measurement_helper::{LayoutOrientation, LayoutWeightModifier, RowColumnMeasureHelper, RowColumnParentDataTrait};

impl Modifier {
    pub fn weight<T>(self, scope: &T, weight: f32) -> Self where T: ?Sized + RowColumnWeightScope {
        scope.weight(self, weight, true)
    }

    pub fn weight_no_fill<T>(self, scope: &T, weight: f32) -> Self where T: ?Sized + RowColumnWeightScope {
        scope.weight(self, weight, false)
    }
}

fn row_column_modifier_element(weight: f32, fill: bool) -> Modifier {
    ModifierNodeElement(
        move || {
            let mut result = LayoutWeightModifier::new();
            result.weight = weight;
            result.fill = fill;

            result
        },
        move |modifier: &mut LayoutWeightModifier| {
            modifier.weight = weight;
            modifier.fill = fill;
        },
    )
}

pub trait RowColumnWeightScope {
    fn weight(&self, modifier: Modifier, weight: f32, fill: bool) -> Modifier {
        modifier.then(row_column_modifier_element(weight, fill))
    }
}

pub(crate) fn row_column_measure_policy(
    layout_orientation: LayoutOrientation,
    arrangement: fn(total_size: usize, size: &[usize], layout_direction: LayoutDirection, arrangement: &mut [i32], density: Density) -> Vec<i32>,
    arrangement_spacing: Dp,
    cross_axis_size: SizeMode,
    cross_axis_alignment: CrossAxisAlignment,
) -> MultiChildrenMeasurePolicy {
    MultiChildrenMeasurePolicyDelegate(move |measure_scope, measurables, constraints| {
        if measurables.is_empty() {
            return measure_scope.layout_without_place(constraints.min_dimension());
        }

        let mut placeables: Vec<Option<Rc<RefCell<dyn Placeable>>>> = vec![None; measurables.len()];

        let row_column_measurement_helper = RowColumnMeasureHelper {
            orientation: layout_orientation,
            arrangement,
            arrangement_spacing,
            cross_axis_size,
            cross_axis_alignment,
        };

        let parent_data = measurables.iter().map(|measurable| {
            measurable.row_column_parent_data().cloned()
        }).collect::<Vec<_>>();

        let measure_result = row_column_measurement_helper.measure_without_placing(measure_scope,
                                                                                   measurables,
                                                                                   &parent_data,
                                                                                   &mut placeables,
                                                                                   constraints,
                                                                                   0..=(measurables.len() - 1));

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

        let layout_direction = measure_scope.get_layout_direction();
        measure_scope.layout((layout_width, layout_height), move |placement_scope| {
            row_column_measurement_helper.place_helper(placement_scope, measure_result, 0, layout_direction, placeables, parent_data)
        })
    })
}
