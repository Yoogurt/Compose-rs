
use crate::foundation::constraint::Constraint;
use crate::foundation::measurable::{Measurable, SingleChildMeasurePolicy};
use crate::foundation::modifier::Modifier;
use crate::foundation::geometry::{CoerceAtLeast, CoerceIn, Dp};
use crate::foundation::layout_receiver::LayoutReceiver;
use crate::foundation::measure_result::MeasureResult;

pub trait SizeModifier {
    fn width(self, width: Dp) -> Modifier;
}

fn size_measure_policy<T>(min_width: T,
                          max_width: T,
                          min_height: T,
                          max_height: T) -> SingleChildMeasurePolicy where T: Into<Dp> + Copy + 'static {
    Box::new(move |layout_receiver: LayoutReceiver, measurable: &mut dyn Measurable, _constraint: &Constraint| -> MeasureResult {
        let target_constraints: Constraint = {
            let max_width = max_width.into();
            let max_width = if max_width.is_unspecific() {
                Constraint::INFINITE
            } else {
                layout_receiver.density.dp_round_to_px(max_width).coerce_at_least(0)
            };

            let max_height = max_height.into();
            let max_height = if max_height.is_unspecific() {
                Constraint::INFINITE
            } else {
                layout_receiver.density.dp_round_to_px(max_height).coerce_at_least(0)
            };

            let min_width = min_width.into();
            let min_width = if min_width.is_unspecific() {
                0
            } else {
                layout_receiver.density.dp_round_to_px(min_width).coerce_in(0..=max_width)
            };

            let min_height = min_height.into();
            let min_height = if min_height.is_unspecific() {
                0
            } else {
                layout_receiver.density.dp_round_to_px(min_height).coerce_in(0..=max_height)
            };

            ((min_width..=max_width), (min_height..=max_height)).into()
        };

        let placeable = measurable.measure(&target_constraints);
        layout_receiver.layout(0, 0, |scope| {
            scope.place_relative(placeable, 0,0)
        })
    })
}

impl SizeModifier for Modifier {
    fn width(self, width: Dp) -> Modifier {
        self.then(Modifier::LayoutModifier {
            measure_policy: size_measure_policy(width, width, Dp::UNSPECIFIC, Dp::UNSPECIFIC),
        })
    }
}