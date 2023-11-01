use std::{cell::RefCell, rc::Rc};
use std::mem::MaybeUninit;
use std::rc::Weak;
use compose_macro::Leak;
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::modifier::Node;
use crate::foundation::modifier_container::ModifierContainer;
use crate::foundation::utils::optional_weak::OptionalWeak;
use super::{modifier::Modifier, parent_data::ParentData, measure_result::MeasureResult, inner_node_coordinator::InnerNodeCoordinator, look_ahead_capable_placeable::NodeCoordinator};

#[derive(Debug)]
pub(crate) struct NodeChain {
    pub(crate) sentine_head: Rc<RefCell<dyn Node>>,

    pub(crate) head: Rc<RefCell<dyn Node>>,
    pub(crate) tail: Rc<RefCell<dyn Node>>,
    pub(crate) modifier_container: Rc<RefCell<ModifierContainer>>,
    pub(crate) parent_data: Option<Box<dyn ParentData>>,
    pub(crate) measure_result: MeasureResult,
    pub(crate) inner_coordinator: Rc<RefCell<InnerNodeCoordinator>>,
    pub(crate) outer_coordinator: Rc<RefCell<dyn NodeCoordinator>>,

    pub(crate) parent: Weak<RefCell<LayoutNode>>,

    pub(crate) layout_node: Weak<RefCell<LayoutNode>>,
}