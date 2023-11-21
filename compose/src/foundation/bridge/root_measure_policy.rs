use crate::foundation::constraint::Constraints;
use crate::foundation::geometry::IntSize;
use crate::foundation::measurable::{Measurable, MultiChildrenMeasurePolicy};
use crate::foundation::measure_result::MeasureResult;
use crate::foundation::measure_scope::{empty_place_action, MeasureScope, MeasureScopeLayoutAction};
use crate::foundation::placement_scope::PlacementScope;

#[inline]
pub(crate) fn root_measure_policy() -> MultiChildrenMeasurePolicy {
    Box::new(
        |measure_scope,
         measurables,
         constraint|
         -> MeasureResult {
            match measurables.len() {
                0 => measure_scope
                    .layout_without_place(constraint.min_dimension()),
                1 => {
                    let (measure_result, placeable) = measurables[0].measure(constraint);
                    measure_scope.layout(
                        measure_result,
                        move |place_scope: &dyn PlacementScope| {
                            place_scope.place_relative(placeable.borrow_mut(), 0, 0);
                        },
                    )
                }
                _ => {
                    let mut max_width = 0;
                    let mut max_height = 0;

                    let mut placeables = measurables
                        .into_iter()
                        .map(|measurable| {
                            let (measure_result, placeable) = measurable.measure(constraint);
                            let size = placeable.borrow().get_size();
                            max_width = max_width.max(size.width);
                            max_height = max_height.max(size.height);
                            placeable
                        })
                        .collect::<Vec<_>>();

                    measure_scope.layout(
                        (constraint.constrain_width(max_width),
                         constraint.constrain_height(max_height)),
                        move |place_scope: &dyn PlacementScope| {
                            placeables
                                .iter_mut()
                                .for_each(|placeable| place_scope.place_relative(placeable.borrow_mut(), 0, 0));
                        },
                    )
                }
            }
        },
    )
}
