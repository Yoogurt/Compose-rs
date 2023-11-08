use crate::foundation::constraint::Constraints;
use crate::foundation::geometry::IntSize;
use crate::foundation::measurable::{Measurable, MultiChildrenMeasurePolicy};
use crate::foundation::measure_result::MeasureResult;
use crate::foundation::measure_scope::MeasureScope;

#[inline]
pub(crate) fn root_measure_policy() -> MultiChildrenMeasurePolicy {
    Box::new(
        |measure_scope: &dyn MeasureScope,
         measurables: &mut [&mut dyn Measurable],
         constraint: &Constraints|
         -> MeasureResult {
            match measurables.len() {
                0 => (constraint.min_width, constraint.min_height).into(),
                1 => {
                    let placeable = measurables[0].measure(constraint);

                    let dimension = placeable.borrow().get_size();
                    measure_scope.layout(
                        dimension,
                        &mut |place_scope| {
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
                            let placeable = measurable.measure(constraint);
                            let size = placeable.borrow().get_size();
                            max_width = max_width.max(size.width());
                            max_height = max_height.max(size.height());
                            placeable
                        })
                        .collect::<Vec<_>>();

                    measure_scope.layout(
                        IntSize::new(constraint.constrain_width(max_width),
                                     constraint.constrain_height(max_height)),
                        &mut |place_scope| {
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
