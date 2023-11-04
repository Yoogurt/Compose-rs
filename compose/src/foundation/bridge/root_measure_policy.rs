use crate::foundation::constraint::Constraints;
use crate::foundation::measurable::{Measurable, MultiChildrenMeasurePolicy};
use crate::foundation::measure_result::MeasureResult;
use crate::foundation::measure_scope::MeasureScope;

#[inline]
pub(crate) fn root_measure_policy() -> MultiChildrenMeasurePolicy {
    Box::new(
        |measure_scope: &mut dyn MeasureScope,
         measurables: &mut [&mut dyn Measurable],
         constraint: &Constraints|
         -> MeasureResult {
            match measurables.len() {
                0 => (constraint.min_width, constraint.min_height).into(),
                1 => {
                    let placeable = measurables[0].measure(constraint);
                    measure_scope.layout(
                        placeable.get_width(),
                        placeable.get_height(),
                        &mut |place_scope| {
                            place_scope.place_relative(placeable, 0, 0);
                        },
                    )
                }
                _ => {
                    let mut max_width = 0;
                    let mut max_height = 0;

                    let mut placeables = measurables
                        .into_iter()
                        .map(|measurable| {
                            let placeable = measurable.measure(constraint);
                            max_width = max_width.max(placeable.get_width());
                            max_height = max_height.max(placeable.get_height());
                            placeable
                        })
                        .collect::<Vec<_>>();

                    measure_scope.layout(
                        constraint.constrain_width(max_width),
                        constraint.constrain_height(max_height),
                        &mut |place_scope| {
                            placeables
                                .iter_mut()
                                .for_each(|placeable| place_scope.place_relative(*placeable, 0, 0));
                        },
                    )
                }
            }
        },
    )
}
