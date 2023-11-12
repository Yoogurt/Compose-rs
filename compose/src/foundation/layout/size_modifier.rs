use crate::foundation::constraint::Constraints;
use crate::foundation::delegatable_node::DelegatableNode;
use crate::foundation::geometry::{CoerceAtLeast, CoerceAtMost, CoerceIn, Dp, IntSize};
use crate::foundation::measurable::{Measurable, SingleChildMeasurePolicy};
use crate::foundation::measure_result::MeasureResult;
use crate::foundation::measure_scope::MeasureScope;
use crate::foundation::modifier::{Modifier, ModifierNode, ModifierNodeImpl, NodeKind, NodeKindPatch};
use crate::foundation::oop::AnyConverter;
use crate::foundation::utils::box_wrapper::WrapWithBox;
use auto_delegate::Delegate;
use std::cell::{RefCell, RefMut};
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};
use compose_foundation_macro::ModifierElement;
use crate::foundation::modifier_node::LayoutModifierNode;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;

pub trait SizeModifier {
    fn width(self, width: Dp) -> Modifier;
    fn height(self, height: Dp) -> Modifier;
}

fn size_measure_policy<T>(
    min_width: T,
    max_width: T,
    min_height: T,
    max_height: T,
) -> SingleChildMeasurePolicy
    where
        T: Into<Dp> + Copy + 'static,
{
    Box::new(
        move |measure_scope: &mut dyn MeasureScope,
              measurable: &mut dyn Measurable,
              _constraint: &Constraints|
              -> MeasureResult {
            let target_constraints: Constraints = {
                let max_width = max_width.into();
                let max_width = if max_width.is_unspecific() {
                    Constraints::INFINITE
                } else {
                    measure_scope
                        .get_density()
                        .dp_round_to_px(max_width)
                        .coerce_at_least(0)
                };

                let max_height = max_height.into();
                let max_height = if max_height.is_unspecific() {
                    Constraints::INFINITE
                } else {
                    measure_scope
                        .get_density()
                        .dp_round_to_px(max_height)
                        .coerce_at_least(0)
                };

                let min_width = min_width.into();
                let min_width = if min_width.is_unspecific() {
                    0
                } else {
                    measure_scope
                        .get_density()
                        .dp_round_to_px(min_width)
                        .coerce_in(0..=max_width)
                };

                let min_height = min_height.into();
                let min_height = if min_height.is_unspecific() {
                    0
                } else {
                    measure_scope
                        .get_density()
                        .dp_round_to_px(min_height)
                        .coerce_in(0..=max_height)
                };

                ((min_width..=max_width), (min_height..=max_height)).into()
            };

            let (measure_result, placeable) = measurable.measure(&target_constraints);
            measure_scope.layout(IntSize::zero(), Box::new(move |scope| scope.place_relative(placeable.borrow_mut(), 0, 0)))
        },
    )
}

#[derive(Debug, Default, Delegate, ModifierElement)]
#[Impl(LayoutModifierNodeConverter)]
struct SizeNode {
    min_width: Dp,
    max_width: Dp,
    min_height: Dp,
    max_height: Dp,
    enforce_incoming: bool,

    #[to(ModifierNode, DelegatableNode)]
    node_impl: ModifierNodeImpl,
}

impl SizeNode {
    fn get_target_constraint(&self, measure_scope: &dyn MeasureScope) -> Constraints {
        let max_width = self.max_width;
        let max_width = if max_width.is_unspecific() {
            Constraints::INFINITE
        } else {
            measure_scope
                .get_density()
                .dp_round_to_px(max_width)
                .coerce_at_least(0)
        };

        let max_height = self.max_height;
        let max_height = if max_height.is_unspecific() {
            Constraints::INFINITE
        } else {
            measure_scope
                .get_density()
                .dp_round_to_px(max_height)
                .coerce_at_least(0)
        };

        let min_width = self.min_width;
        let min_width = if min_width.is_unspecific() {
            0
        } else {
            measure_scope
                .get_density()
                .dp_round_to_px(min_width)
                .coerce_in(0..=max_width)
        };

        let min_height = self.min_height;
        let min_height = if min_height.is_unspecific() {
            0
        } else {
            measure_scope
                .get_density()
                .dp_round_to_px(min_height)
                .coerce_in(0..=max_height)
        };

        ((min_width..=max_width), (min_height..=max_height)).into()
    }
}

impl LayoutModifierNode for SizeNode {
    fn measure(
        &self,
        measure_scope: &mut dyn MeasureScope,
        measurable: &mut dyn Measurable,
        constraints: &Constraints,
    ) -> MeasureResult {
        let target_constraints = &self.get_target_constraint(measure_scope);

        let wrapped_constraints = if self.enforce_incoming {
            constraints.constrain(&target_constraints)
        } else {
            let resolved_min_width = if self.min_width != Dp::UNSPECIFIC {
                target_constraints.min_width
            } else {
                constraints
                    .min_width
                    .coerce_at_most(target_constraints.max_width)
            };

            let resolved_max_width = if self.max_width != Dp::UNSPECIFIC {
                target_constraints.max_width
            } else {
                constraints
                    .max_width
                    .coerce_at_least(target_constraints.min_width)
            };

            let resolved_min_height = if self.min_height != Dp::UNSPECIFIC {
                target_constraints.min_height
            } else {
                constraints
                    .min_height
                    .coerce_at_most(target_constraints.max_height)
            };

            let resolved_max_height = if self.max_height != Dp::UNSPECIFIC {
                target_constraints.max_height
            } else {
                constraints
                    .max_height
                    .coerce_at_least(target_constraints.min_height)
            };

            (
                (resolved_min_width..=resolved_max_width),
                (resolved_min_height..=resolved_max_height),
            )
                .into()
        };
        let (measure_result, placeable) = measurable.measure(&wrapped_constraints);

        measure_scope.layout(
            measure_result.into(),
            Box::new(move |scope| scope.place_relative(placeable.borrow_mut(), 0, 0)),
        )
    }
}

impl NodeKindPatch for SizeNode {
    fn get_node_kind(&self) -> NodeKind {
        NodeKind::Layout
    }
}

impl PartialEq for SizeNode {
    fn eq(&self, other: &Self) -> bool {
        self.min_width == other.min_width
            && self.max_width == other.max_width
            && self.min_height == other.min_height
            && self.max_height == other.max_height
            && self.enforce_incoming == other.enforce_incoming
    }
}

impl Hash for SizeNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.min_width.hash(state);
        self.max_width.hash(state);
        self.min_height.hash(state);
        self.max_height.hash(state);
        self.enforce_incoming.hash(state);
    }
}

fn size_element<T>(
    min_width_raw: T,
    max_width_raw: T,
    min_height_raw: T,
    max_height_raw: T,
    enforce_incoming: bool,
) -> Modifier
    where
        T: Into<Dp> + Copy + 'static,
{
    Modifier::ModifierNodeElement {
        create: (move || {
            SizeNode {
                min_width: min_width_raw.into(),
                max_width: max_width_raw.into(),
                min_height: min_height_raw.into(),
                max_height: max_height_raw.into(),
                enforce_incoming,
                ..Default::default()
            }
                .wrap_with_rc_refcell() as Rc<RefCell<dyn ModifierNode>>
        })
            .wrap_with_box(),
        update: (move |mut size_node: RefMut<dyn ModifierNode>| {
            if let Some(size_node) = size_node.as_any_mut().downcast_mut::<SizeNode>() {
                size_node.min_width = min_width_raw.into();
                size_node.max_width = max_width_raw.into();
                size_node.min_height = min_height_raw.into();
                size_node.max_height = max_height_raw.into();
                size_node.enforce_incoming = enforce_incoming;
            } else {
                panic!("wrong type for SizeNode");
            }
        })
            .wrap_with_box(),
    }
}

impl SizeModifier for Modifier {
    fn width(self, width: Dp) -> Modifier {
        self.then(size_element(
            width,
            width,
            Dp::UNSPECIFIC,
            Dp::UNSPECIFIC,
            true,
        ))
    }

    fn height(self, height: Dp) -> Modifier {
        self.then(size_element(
            Dp::UNSPECIFIC,
            Dp::UNSPECIFIC,
            height,
            height,
            true,
        ))
    }
}
