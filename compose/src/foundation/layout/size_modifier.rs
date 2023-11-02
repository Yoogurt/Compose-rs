use std::any::Any;
use std::cell::RefCell;
use std::ffi::c_void;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use auto_delegate::Delegate;
use crate::foundation::constraint::Constraint;
use crate::foundation::delegatable_node::DelegatableNode;
use crate::foundation::measurable::{Measurable, SingleChildMeasurePolicy};
use crate::foundation::modifier::{Modifier, Node, NodeImpl, NodeKind, NodeKindPatch};
use crate::foundation::geometry::{CoerceAtLeast, CoerceIn, Dp};
use crate::foundation::layout_modifier_node::LayoutModifierNode;
use crate::foundation::measure_scope::MeasureScope;
use crate::foundation::measure_result::MeasureResult;
use crate::foundation::utils::box_wrapper::WrapWithBox;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;

pub trait SizeModifier {
    fn width(self, width: Dp) -> Modifier;
}

fn size_measure_policy<T>(min_width: T,
                          max_width: T,
                          min_height: T,
                          max_height: T) -> SingleChildMeasurePolicy where T: Into<Dp> + Copy + 'static {
    Box::new(move |measure_scope: &mut dyn MeasureScope, measurable: &mut dyn Measurable, _constraint: &Constraint| -> MeasureResult {
        let target_constraints: Constraint = {
            let max_width = max_width.into();
            let max_width = if max_width.is_unspecific() {
                Constraint::INFINITE
            } else {
                measure_scope.get_density().dp_round_to_px(max_width).coerce_at_least(0)
            };

            let max_height = max_height.into();
            let max_height = if max_height.is_unspecific() {
                Constraint::INFINITE
            } else {
                measure_scope.get_density().dp_round_to_px(max_height).coerce_at_least(0)
            };

            let min_width = min_width.into();
            let min_width = if min_width.is_unspecific() {
                0
            } else {
                measure_scope.get_density().dp_round_to_px(min_width).coerce_in(0..=max_width)
            };

            let min_height = min_height.into();
            let min_height = if min_height.is_unspecific() {
                0
            } else {
                measure_scope.get_density().dp_round_to_px(min_height).coerce_in(0..=max_height)
            };

            ((min_width..=max_width), (min_height..=max_height)).into()
        };

        let placeable = measurable.measure(&target_constraints);
        measure_scope.layout(0, 0, &mut | scope| {
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

impl DelegatableNode for SizeNode {}

impl LayoutModifierNode for SizeNode {
    fn measure(&mut self, measure_scope: &mut dyn MeasureScope, measurable: &dyn Measurable, constraint: &Constraint) {
        todo!()
    }
}

impl NodeKindPatch for SizeNode {
    fn get_node_kind(&mut self) -> NodeKind {
        NodeKind::LayoutModifierNode(self)
    }
}

impl PartialEq for SizeNode {
    fn eq(&self, other: &Self) -> bool {
        self.min_width == other.min_width
            && self.max_width == other.max_width
            && self.min_height == other.min_height
            && self.max_height == other.max_height
    }
}

impl Hash for SizeNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.min_width.hash(state);
        self.max_width.hash(state);
        self.min_height.hash(state);
        self.max_height.hash(state);
    }
}

fn size_element<T>(min_width_raw: T,
                   max_width_raw: T,
                   min_height_raw: T,
                   max_height_raw: T) -> Modifier where T: Into<Dp> + Copy + 'static {

    Modifier::ModifierNodeElement {
        create: (move || {
            SizeNode {
                min_width: min_width_raw.into(),
                max_width: max_width_raw.into(),
                min_height: min_height_raw.into(),
                max_height: max_height_raw.into(),
                ..Default::default()
            } .wrap_with_rc_refcell() as Rc<RefCell<dyn Node>>
        }).wrap_with_box(),
        update: (move |size_node_rc: &Rc<RefCell<dyn Node>>| {
            let mut size_node_mut = size_node_rc.borrow_mut();
            let size_node = size_node_mut.deref_mut();

            if let Some(size_node) = size_node.as_any_mut().downcast_mut::<SizeNode>() {
                size_node.min_width = min_width_raw.into();
                size_node.max_width = max_width_raw.into();
                size_node.min_height = min_height_raw.into();
                size_node.max_height = max_height_raw.into();
            } else {
                panic!("wrong type for SizeNode");
            }
        }).wrap_with_box(),
    }
}

impl SizeModifier for Modifier {
    fn width(self, width: Dp) -> Modifier {
        self.then(size_element(width, width, Dp::UNSPECIFIC, Dp::UNSPECIFIC))
    }
}
