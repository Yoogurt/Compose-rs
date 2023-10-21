use crate::foundation::{MeasurePolicyDelegate, MeasureResult};

#[inline]
pub(crate) fn rootMeasurePolicy() -> MeasurePolicyDelegate {
    |measurables, constraint| {
        match measurables.len() {
            0 => {
                (constraint.min_width, constraint.min_height).into()
            }
            1 => {
                let placeable = measurables[0].measure(constraint);
                (placeable.width, placeable.height).into()
            }
            _ => {
                let mut max_width = 0;
                let mut max_height = 0;

                let placeables = measurables.iter().map(|&measurable| {
                    let placeable = measurable.measure(constraint);
                                max_width = max_width.max(placeable.width);
                    max_height = max_height.max(placeable.height);
                    placeable
                });

                (max_width, max_height).into()
            }
        }

    }
}