use crate::foundation::constraint::Constraint;
use crate::foundation::layout_receiver::LayoutReceiver;
use crate::foundation::measurable::{Measurable, MultiChildrenMeasurePolicy};
use crate::foundation::measure_result::MeasureResult;

#[inline]
pub(crate) fn root_measure_policy() -> MultiChildrenMeasurePolicy {
    Box::new(|_layout_receiver: LayoutReceiver, measurables:&mut [&mut dyn Measurable], constraint: &Constraint| -> MeasureResult {
        match measurables.len() {
            0 => {
                (constraint.min_width, constraint.min_height).into()
            }
            1 => {
                let placeable = measurables[0].measure(constraint);
                (placeable.get_width(), placeable.get_height()).into()
            }
            _ => {
                let mut max_width = 0;
                let mut max_height = 0;

                let _placeables = measurables.into_iter().map(|measurable| {
                    let placeable = measurable.measure(constraint);
                                max_width = max_width.max(placeable.get_width());
                    max_height = max_height.max(placeable.get_height());
                    placeable
                });

                (max_width, max_height).into()
            }
        }
    })
}