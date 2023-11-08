use std::any::Any;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use auto_delegate::Delegate;
use compose_macro::Composable;
use compose_foundation_macro::ModifierElement;

use crate::foundation::modifier::{ModifierNode, ModifierNodeImpl, NodeKind, NodeKindPatch};
use crate::foundation::placeable::Placeable;
use crate::foundation::utils::box_wrapper::WrapWithBox;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;
use crate::foundation::{
    constraint::Constraints, measurable::Measurable, measure_result::MeasureResult,
    measure_scope::MeasureScope, modifier::Modifier,
};
use crate::{self as compose};
use crate::foundation::delegatable_node::DelegatableNode;
use crate::foundation::oop::AnyConverter;
use crate::foundation::oop::LayoutModifierNodeConverter;
use crate::foundation::parent_data_modifier_node::ParentDataModifierNode;
use crate::foundation::ui::align::Alignment;

use crate::widgets::layout::Layout;

// #[macro_export]
// macro_rules! Box {
//     ( $modifier_expr:expr, $fn_body:tt ) => {
//         compose::widgets::r#box::box_internal($modifier_expr, || {
//              $fn_body
//         });
//     };
//
//     ( $fn_body:tt ) => {
//         compose::widgets::r#box::box_internal(std::default::Default::default(), || {
//              $fn_body
//         });
//     };
// }

trait BoxMeasurableTrait {
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

struct BoxScopeInstance {}

impl BoxScope for BoxScopeInstance {}

const INSTANCE: &dyn BoxScope = &BoxScopeInstance {};

impl BoxMeasurableTrait for &mut dyn Measurable {
    fn matches_parent_size(&self) -> bool {
        if let Some(parent_data) = self.get_parent_data() {
            let box_child_data_node = parent_data.downcast_ref::<BoxChildDataNode>();

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
struct BoxChildDataNode {
    alignment: Alignment,
    match_parent_size: bool,
    #[to(ModifierNode)]
    node_impl: ModifierNodeImpl,
}

impl DelegatableNode for BoxChildDataNode {}

impl NodeKindPatch for BoxChildDataNode {
    fn get_node_kind(& self) -> NodeKind {
        NodeKind::ParentData
    }
}

impl ParentDataModifierNode for BoxChildDataNode {
    fn modify_parent_data(&mut self, parent_data: Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        todo!()
    }
}

fn box_child_data(alignment: Alignment, match_parent_size: bool) -> Modifier {
    Modifier::ModifierNodeElement {
        create: (move || {
            let mut box_child_data_node = BoxChildDataNode::default();

            box_child_data_node.alignment = alignment;
            box_child_data_node.match_parent_size = match_parent_size;

            box_child_data_node.wrap_with_rc_refcell() as Rc<RefCell<dyn ModifierNode>>
        }).wrap_with_box(),
        update: (move |mut box_child_data_node: RefMut<dyn ModifierNode>| {
            if let Some(box_child_data_node) = box_child_data_node.as_any_mut().downcast_mut::<BoxChildDataNode>() {
                box_child_data_node.alignment = alignment;
                box_child_data_node.match_parent_size = match_parent_size;
            }
        }).wrap_with_box(),
    }
}

fn place_in_box(placeable: &mut dyn Placeable) {}

fn box_measure_policy(
    measure_scope: & dyn MeasureScope,
    measurables: &mut [&mut dyn Measurable],
    constraints: &Constraints,
) -> MeasureResult {
    let children_count = measurables.len();
    match children_count {
        0 => measure_scope.layout((constraints.min_width, constraints.min_height).into(), &mut |_| {}),
        1 => {
            let placeable = measurables[0].measure(constraints);
let dimension = placeable.borrow().get_size();
            measure_scope.layout(
                dimension,
                &mut |scope| scope.place_relative(placeable.borrow_mut(), 0, 0),
            )
        }
        _ => {
            let mut placeables: Vec<Option<Rc<RefCell<dyn Placeable>>>> =
                Vec::with_capacity(measurables.len());

            let mut has_match_parent_size_children = false;
            let (mut box_width, mut box_height) = constraints.max_dimension();

            {
                measurables
                    .iter_mut()
                    .enumerate()
                    .for_each(|(index, measurable)| {
                        if measurable.matches_parent_size() {
                            has_match_parent_size_children = true
                        } else {
                            let placeable = measurable.measure(&constraints).borrow().get_size();
                            box_width = box_width.max(placeable.width());
                            box_height = box_height.max(placeable.height());
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
                                Some(measurable.measure(&match_parent_size_constraints));
                        } else {
                            placeables[index] = Some(measurable.as_placeable());
                        }
                    });
            }

            measure_scope.layout((box_width, box_height).into(), &mut |scope| {
                placeables.iter_mut().for_each(|placeable| {})
            })
        }
    }
}

#[Composable]
pub fn BoxLayout(modifier: Modifier, mut content: impl FnMut(&dyn BoxScope)) {
    Layout(modifier, box_measure_policy, || {
        let box_scope = BoxScopeInstance {};
        content(&box_scope);
    });
}
