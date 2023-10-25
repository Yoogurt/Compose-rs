use crate::foundation::MultiChildrenMeasurePolicy;

#[inline]
pub(crate) fn root_measure_policy() -> MultiChildrenMeasurePolicy {
    |_layout_receiver, measurables, constraint| {
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
    }
}