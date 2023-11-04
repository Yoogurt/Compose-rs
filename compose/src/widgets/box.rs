use compose_macro::Composable;

use crate::foundation::{measure_scope::MeasureScope, measurable::Measurable, constraint::Constraints, measure_result::MeasureResult, modifier::Modifier};
use crate::{self as compose};
use crate::foundation::layout_modifier_node::LayoutModifierNode;
use crate::foundation::modifier::Node;
use crate::foundation::placeable::Placeable;
use crate::foundation::utils::box_wrapper::WrapWithBox;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;

use crate::widgets::layout::Layout;

#[macro_export]
macro_rules! Box {
    ( $modifier_expr:expr, $($fn_body:tt)* ) => {
        compose::widgets::r#box::box_internal($modifier_expr, || {
             $($fn_body)*
        });
    };

    ( $($fn_body:tt)* ) => {
        compose::widgets::r#box::box_internal(std::default::Default::default(), || {
             $($fn_body)*
        });
    };
}

trait BoxMeasurableTrait {
    fn matches_parent_size(&self) -> bool;
}

impl BoxMeasurableTrait for &mut dyn Measurable {
    fn matches_parent_size(&self) -> bool {
        if let Some(parent_data) = self.get_parent_data() {}

        false
    }
}

struct BoxChildDataNode {
    match_parent_size: bool,
}


fn box_child_data_element(match_parent_size: bool) -> Modifier {
    // Modifier::ModifierNodeElement {
    //     create: move || {
    //         todo!()
    //     }.wrap_with_box(),
    //     update: move |_| {
    //         todo!()
    //     }.wrap_with_box(),
    // }
    todo!()
}

fn place_in_box(placeable: &mut dyn Placeable) {}

fn box_measure_policy(measure_scope: &mut dyn MeasureScope, measurables: &mut [&mut dyn Measurable], constraints: &Constraints) -> MeasureResult {
    let children_count = measurables.len();
    match children_count {
        0 => { measure_scope.layout(constraints.min_width, constraints.min_height, &mut |_| {}) }
        1 => {
            let placeable = measurables[0].measure(constraints);
            measure_scope.layout(placeable.get_width(), placeable.get_height(), &mut |scope| {
                scope.place_relative(placeable, 0, 0)
            })
        }
        _ => {
            let mut placeables: Vec<Option<&mut dyn Placeable>> = Vec::with_capacity(measurables.len());

            let mut has_match_parent_size_children = false;
            let (mut box_width, mut box_height) = constraints.max_dimension();

            {
                measurables.iter_mut().enumerate().for_each(|(index, measurable)| {
                    if measurable.matches_parent_size() {
                        has_match_parent_size_children = true
                    } else {
                        let placeable = measurable.measure(&constraints);
                        box_width = box_width.max(placeable.get_width());
                        box_height = box_height.max(placeable.get_height());
                    }
                });
            }

            if has_match_parent_size_children {
                let match_parent_size_constraints = Constraints::from(
                    (if box_width != Constraints::INFINITE { box_width } else { 0 }..=box_width,
                     if box_height != Constraints::INFINITE { box_height } else { 0 }..=box_height));

                measurables.iter_mut().enumerate().for_each(|(index, measurable)| {
                    if measurable.matches_parent_size() {
                        placeables[index] = Some(measurable.measure(&match_parent_size_constraints));
                    } else {
                        placeables[index] = Some(measurable.as_placeable_mut());
                    }
                });
            }

            measure_scope.layout(box_width, box_height, &mut |scope| {
                placeables.iter_mut().for_each(|placeable| {})
            })
        }
    }
}

#[Composable]
pub fn box_internal(modifier: Modifier, content: fn()) {
    Layout(modifier, box_measure_policy, content);
}