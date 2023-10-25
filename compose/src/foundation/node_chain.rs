use std::{cell::RefCell, rc::Rc};
use super::{modifier::Modifier, parent_data::ParentData, measure_result::MeasureResult, inner_coodinator::InnerCoordinator, outer_coordinator::OuterCoordinator, look_ahead_capable_placeable::LayoutNodeWrapper};

#[derive(Debug)]
pub(crate) struct NodeChain {
    pub(crate) modifier: Modifier,
    pub(crate) parent_data: Option<Box<dyn ParentData>>,
    pub(crate) measure_result: MeasureResult,
    pub(crate) inner_placeable: Rc<RefCell<InnerCoordinator>>,
    pub(crate) outer_measurable_placeable: OuterCoordinator,
    pub(crate) outer_layout_node: Rc<RefCell<dyn LayoutNodeWrapper>>,
}