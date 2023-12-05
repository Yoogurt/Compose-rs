use crate::foundation::layout::layout_id::ParentDataLayoutId;
use std::any::Any;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

use auto_delegate::Delegate;
use compose_foundation_macro::ModifierElement;
use compose_macro::Composable;

use crate as compose;
use crate::foundation::{
    constraint::Constraints, measurable::Measurable,
    measure_scope::MeasureScope, modifier::Modifier,
};
use crate::foundation::geometry::{Density, IntSize};
use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::measurable::{MultiChildrenMeasurePolicy, MultiChildrenMeasurePolicyDelegate};
use crate::foundation::measure_scope::{empty_place_action, MeasureScopeLayoutAction};
use crate::foundation::modifier::{modifier_node_element_creator, modifier_node_element_updater, ModifierNode, ModifierNodeImpl, NodeKind, NodeKindPatch};
use crate::foundation::modifier_node::ParentDataModifierNode;
use crate::foundation::parent_data::ExtractParentData;
use crate::foundation::placeable::Placeable;
use crate::foundation::placement_scope::PlacementScope;
use crate::foundation::ui::align::Alignment;
use crate::foundation::utils::box_wrapper::WrapWithBox;
use crate::foundation::utils::option_extension::{OptionalInstanceConverter, OptionThen};
use crate::impl_node_kind_parent_data;
use crate::widgets::layout::Layout;

trait BoxMeasurableTrait {
    fn box_child_data_node(&self) -> Option<&BoxChildDataNode>;
    fn alignment(&self) -> Option<Alignment>;
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
    fn box_child_data_node(&self) -> Option<&BoxChildDataNode> {
        self.cast::<BoxChildDataNode>()
    }

    fn alignment(&self) -> Option<Alignment> {
        self.box_child_data_node().map(|child_data| child_data.alignment)
    }

    fn matches_parent_size(&self) -> bool {
        self.box_child_data_node().map(|child_data| child_data.match_parent_size).unwrap_or(false)
    }
}

#[derive(Debug, Default, Clone)]
struct BoxChildDataNode {
    alignment: Alignment,
    match_parent_size: bool,
}

#[derive(Delegate, Debug, Default, ModifierElement)]
#[Impl(ParentData)]
struct BoxChildDataModifierNode {
    box_child_data_node: BoxChildDataNode,

    #[to(ModifierNode)]
    node_impl: ModifierNodeImpl,
}

impl_node_kind_parent_data!(BoxChildDataModifierNode);

impl ParentDataModifierNode for BoxChildDataModifierNode {
    fn modify_parent_data(&mut self, _: Density, parent_data: Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        let mut parent_data = parent_data.cast_or(|| self.box_child_data_node.clone());
        parent_data.match_parent_size = self.box_child_data_node.match_parent_size;
        parent_data.alignment = self.box_child_data_node.alignment;
        Some(parent_data)
    }
}

fn box_child_data(alignment: Alignment, match_parent_size: bool) -> Modifier {
    Modifier::ModifierNodeElement {
        create: modifier_node_element_creator(move || {
            let mut box_child_data_node = BoxChildDataModifierNode::default();

            box_child_data_node.box_child_data_node.alignment = alignment;
            box_child_data_node.box_child_data_node.match_parent_size = match_parent_size;

            box_child_data_node
        }),
        update: modifier_node_element_updater(move |box_child_data_node: &mut BoxChildDataModifierNode| {
            box_child_data_node.box_child_data_node.alignment = alignment;
            box_child_data_node.box_child_data_node.match_parent_size = match_parent_size;
        }),
    }
}

fn place_in_box(placeable: &mut dyn Placeable,
                layout_direction: LayoutDirection,
                box_size: IntSize,
                alignment: Alignment) {
    let position = alignment.align(placeable.get_size(), box_size, layout_direction);
    placeable.place_at(position, 0.0);
}

fn remember_box_measure_policy(alignment: Alignment, propagate_min_constraint: bool) -> MultiChildrenMeasurePolicy {
    MultiChildrenMeasurePolicyDelegate(move |measure_scope: &dyn MeasureScope,
                                             measurables: &mut [&mut dyn Measurable],
                                             constraints: &Constraints| {
        let children_count = measurables.len();

        let content_constraints = if propagate_min_constraint {
            *constraints
        } else {
            Constraints::new(0..=constraints.max_width, 0..=constraints.max_height)
        };
        match children_count {
            0 => measure_scope.layout((constraints.min_width, constraints.min_height), empty_place_action),
            1 => {
                let (measure_result, placeable) = measurables[0].measure(&content_constraints);
                measure_scope.layout(
                    measure_result,
                    move |scope: &dyn PlacementScope| scope.place_relative(placeable.borrow_mut(), 0, 0),
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
                    child.alignment().unwrap_or(alignment)
                }).collect::<Vec<Alignment>>();

                measure_scope.layout((box_width, box_height), (move |scope| {
                    placeables.iter_mut().enumerate().for_each(|(index, placeable)| {
                        let mut placeable = placeable.as_mut().unwrap().borrow_mut();

                        place_in_box(placeable.deref_mut(),
                                     layout_direction,
                                     scope.parent_size(),
                                     box_child_data[index],
                        );
                    })
                }))
            }
        }
    })
}

#[Composable]
pub fn BoxLayout(modifier: Modifier, mut content: impl FnMut(&dyn BoxScope)) {
    Layout(modifier, remember_box_measure_policy(Alignment::TOP_START, false), || {
        content(INSTANCE);
    });
}