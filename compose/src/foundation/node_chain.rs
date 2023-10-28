use std::{cell::RefCell, rc::Rc};
use std::mem::MaybeUninit;
use std::rc::Weak;
use crate::foundation::layout_node::LayoutNode;
use super::{modifier::Modifier, parent_data::ParentData, measure_result::MeasureResult, inner_node_coordinator::InnerNodeCoordinator, look_ahead_capable_placeable::NodeCoordinator};

#[derive(Debug)]
pub(crate) struct NodeChain {
    pub(crate) modifier: Rc<RefCell<Modifier>>,
    pub(crate) parent_data: Option<Box<dyn ParentData>>,
    pub(crate) measure_result: MeasureResult,
    pub(crate) inner_coordinator: Rc<RefCell<InnerNodeCoordinator>>,
    pub(crate) outer_coordinator: Rc<RefCell<dyn NodeCoordinator>>,

    pub(crate) layout_node: MaybeUninit<Weak<RefCell<LayoutNode>>>,
}