use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::ops::DerefMut;
use std::rc::{Rc, Weak};
use crate::foundation::remember::remember;
use auto_delegate::Delegate;
use compose_foundation_macro::{AnyConverter, ModifierElement};
use compose_macro::Composable;

use crate as compose;
use crate::foundation::{
    constraint::Constraints, measurable::Measurable,
    measure_scope::MeasureScope, modifier::Modifier,
};
use crate::foundation::delegatable_node::DelegatableNode;
use crate::foundation::geometry::Density;
use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::measurable::MultiChildrenMeasurePolicy;
use crate::foundation::modifier::{ModifierNode, ModifierNodeImpl, NodeKind, NodeKindPatch};
use crate::foundation::modifier_node::ParentDataModifierNode;
use crate::foundation::oop::AnyConverter;
use crate::foundation::placeable::Placeable;
use crate::foundation::placement_scope::PlacementScope;
use crate::foundation::ui::align::Alignment;
use crate::foundation::utils::box_wrapper::WrapWithBox;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;
use crate::foundation::utils::self_reference::SelfReference;
use crate::widgets::layout::Layout;

trait BoxMeasurableTrait {
    fn box_child_data_node(&self) -> Option<Ref<BoxChildDataNode>>;
    fn matches_parent_size(&self) -> bool;
}

pub trait BoxScope {
    fn align(&self, modifier: Modifier, alignment: Alignment) -> Modifier {
        modifier.then(box_child_data(alignment, false))
    }

    fn match_parent_size(&self, modifier: Modifier) -> Modifier {
        modifier.then(box_child_data(Alignment::CENTER, true))
    }
}

impl Modifier {
    pub fn align(self, box_scope: &dyn BoxScope, alignment: Alignment) -> Modifier {
        box_scope.align(self, alignment)
    }

    pub fn match_parent_size(self, box_scope: &dyn BoxScope) -> Modifier {
        box_scope.match_parent_size(self)
    }
}

struct BoxScopeInstance {}

impl BoxScope for BoxScopeInstance {}

const INSTANCE: &dyn BoxScope = &BoxScopeInstance {};

impl BoxMeasurableTrait for &mut dyn Measurable {
    fn box_child_data_node(&self) -> Option<Ref<BoxChildDataNode>> {
        self.get_parent_data_ref()
            .and_then(|parent_data| Ref::filter_map(parent_data.borrow(), |parent_data| {
                parent_data.downcast_ref::<BoxChildDataNode>()
            }).ok())
    }

    fn matches_parent_size(&self) -> bool {
        if let Some(parent_data) = self.get_parent_data() {
            let box_child_data_node = self.box_child_data_node();

            return if let Some(node) = box_child_data_node {
                node.match_parent_size
            } else {
                false
            };
        }
        false
    }
}

#[derive(Delegate, Debug, Default, ModifierElement)]
#[Impl(ParentDataModifierNodeConverter)]
struct BoxChildDataNode {
    alignment: Alignment,
    match_parent_size: bool,

    #[to(ModifierNode, DelegatableNode)]
    node_impl: ModifierNodeImpl,
    weak_self: Weak<RefCell<Self>>,
}

impl NodeKindPatch for BoxChildDataNode {
    fn get_node_kind(&self) -> NodeKind {
        NodeKind::ParentData
    }
}

impl SelfReference for BoxChildDataNode {
    fn get_self(&self) -> Weak<RefCell<Self>> {
        self.weak_self.clone()
    }
}

impl ParentDataModifierNode for BoxChildDataNode {
    fn modify_parent_data(&mut self, density: Density, parent_data: Option<Rc<RefCell<dyn Any>>>) -> Option<Rc<RefCell<dyn Any>>> {
        self.get_self().upgrade().map(|this| this as Rc<RefCell<dyn Any>>)
    }
}

fn box_child_data(alignment: Alignment, match_parent_size: bool) -> Modifier {
    Modifier::ModifierNodeElement {
        create: (move || {
            let mut box_child_data_node = BoxChildDataNode::default();

            box_child_data_node.alignment = alignment;
            box_child_data_node.match_parent_size = match_parent_size;

            let box_child_data_node = box_child_data_node.wrap_with_rc_refcell();
            box_child_data_node.borrow_mut().weak_self = Rc::downgrade(&box_child_data_node);

            box_child_data_node as Rc<RefCell<dyn ModifierNode>>
        }).wrap_with_box(),
        update: (move |mut box_child_data_node: RefMut<dyn ModifierNode>| {
            if let Some(box_child_data_node) = box_child_data_node.as_any_mut().downcast_mut::<BoxChildDataNode>() {
                box_child_data_node.alignment = alignment;
                box_child_data_node.match_parent_size = match_parent_size;
            }
        }).wrap_with_box(),
    }
}

fn place_in_box(placeable: &mut dyn Placeable,
                layout_direction: LayoutDirection,
                box_width: usize, box_height: usize,
                alignment: Alignment) {
    let position = alignment.align(placeable.get_size(), (box_width, box_height).into(), layout_direction);
    placeable.place_at((position.x, position.y).into(), 0.0);
}

fn remember_box_measure_policy(alignment: Alignment, propagate_min_constraint: bool) -> MultiChildrenMeasurePolicy {
    (move |measure_scope: &dyn MeasureScope,
           measurables: &mut [&mut dyn Measurable],
           constraints: &Constraints| {
        let children_count = measurables.len();

        let content_constraints = if propagate_min_constraint {
            *constraints
        } else {
            Constraints::new(0..=constraints.max_width, 0..=constraints.max_height)
        };
        match children_count {
            0 => measure_scope.layout((constraints.min_width, constraints.min_height).into(), (|_: &dyn PlacementScope| {}).wrap_with_box()),
            1 => {
                let (measure_result, placeable) = measurables[0].measure(&content_constraints);
                measure_scope.layout(
                    measure_result,
                    (move |scope: &dyn PlacementScope| scope.place_relative(placeable.borrow_mut(), 0, 0)).wrap_with_box(),
                )
            }
            _ => {
                let mut placeables: Vec<Option<Rc<RefCell<dyn Placeable>>>> =
                    vec![None; children_count];

                let mut has_match_parent_size_children = false;
                let (mut box_width, mut box_height) = content_constraints.max_dimension();

                {
                    measurables
                        .iter_mut()
                        .enumerate()
                        .for_each(|(index, measurable)| {
                            if measurable.matches_parent_size() {
                                has_match_parent_size_children = true
                            } else {
                                let (measure_result, _) = measurable.measure(&content_constraints);
                                box_width = box_width.max(measure_result.width);
                                box_height = box_height.max(measure_result.height);
                            }
                        });
                }

                if has_match_parent_size_children {
                    let match_parent_size_constraints = Constraints::from((
                        if box_width != Constraints::INFINITE {
                            box_width
                        } else {
                            0
                        }..=box_width,
                        if box_height != Constraints::INFINITE {
                            box_height
                        } else {
                            0
                        }..=box_height,
                    ));

                    measurables
                        .iter_mut()
                        .enumerate()
                        .for_each(|(index, measurable)| {
                            if measurable.matches_parent_size() {
                                placeables[index] =
                                    Some(measurable.measure(&match_parent_size_constraints).1);
                            } else {
                                placeables[index] = Some(measurable.as_placeable());
                            }
                        });
                } else {
                    measurables
                        .iter_mut()
                        .enumerate()
                        .for_each(|(index, measurable)| {
                            placeables[index] = Some(measurable.as_placeable());
                        });
                }

                let layout_direction = measure_scope.get_layout_direction();
                let box_child_data = measurables.iter().map(|child| {
                    child.box_child_data_node().map(|box_child_data_node| box_child_data_node.alignment).unwrap_or(alignment)
                }).collect::<Vec<Alignment>>();

                measure_scope.layout((box_width, box_height).into(), (move |scope: &dyn PlacementScope| {
                    placeables.iter_mut().enumerate().for_each(|(index, placeable)| {
                        place_in_box(placeable.as_mut().unwrap().borrow_mut().deref_mut(),
                                     layout_direction,
                                     box_width,
                                     box_height,
                                     box_child_data[index],
                        );
                    })
                }).wrap_with_box())
            }
        }
    }).wrap_with_box()
}

#[Composable]
pub fn BoxLayout(modifier: Modifier, mut content: impl FnMut(&dyn BoxScope)) {
    Layout(modifier, remember_box_measure_policy(Alignment::TOP_START, false), || {
        content(INSTANCE);
    });
}