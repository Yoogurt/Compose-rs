use std::any::Any;
use std::ops::DerefMut;
use auto_delegate::Delegate;
use crate::foundation::constraint::Constraint;
use crate::foundation::measurable::{Measurable, SingleChildMeasurePolicy};
use crate::foundation::modifier::{Modifier, Node, NodeImpl};
use crate::foundation::geometry::{CoerceAtLeast, CoerceIn, Dp};
use crate::foundation::layout_receiver::LayoutReceiver;
use crate::foundation::measure_result::MeasureResult;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;

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
            scope.place_relative(placeable, 0, 0)
        })
    })
}

#[derive(Debug, Default, Delegate)]
struct SizeNode {
    min_width: Dp,
    max_width: Dp,
    min_height: Dp,
    max_height: Dp,

    #[to(Node)]
    node_impl: NodeImpl,
}

fn size_element<T>(min_width_raw: T,
                   max_width_raw: T,
                   min_height_raw: T,
                   max_height_raw: T) -> Modifier where T: Into<Dp> + Copy + 'static {
    Modifier::ModifierNodeElement {
        create: Box::new(move || {
            SizeNode {
                min_width: min_width_raw.into(),
                max_width: max_width_raw.into(),
                min_height: min_height_raw.into(),
                max_height: max_height_raw.into(),
                ..Default::default()
            }.wrap_with_rc_refcell()
        }),
        update: Box::new(|size_node_rc| {
            let mut size_node_mut = size_node_rc.borrow_mut();
            let mut a = size_node_mut.deref_mut();
            SizeNode::as_any_mut(a);
            // if let Some(size_node) = SizeNode::as_any_mut(a).downcast_mut::<SizeNode>() {
            //
            // } else {
            //     panic!("wrong type for SizeNode");
            // }
        }),
    }
}

impl SizeModifier for Modifier {
    fn width(self, width: Dp) -> Modifier {
        self.then(size_element(width, width, Dp::UNSPECIFIC, Dp::UNSPECIFIC))
    }
}
