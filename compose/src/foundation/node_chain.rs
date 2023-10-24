#[derive(Debug)]
pub(crate) struct NodeChain {
    modifier: Modifier,
    parent_data: Option<Box<dyn ParentData>>,
    measure_result: MeasureResult,
    inner_placeable: Rc<RefCell<InnerCoordinator>>,
    outer_measurable_placeable: OuterCoordinator,
    outer_layout_node: Rc<RefCell<dyn LayoutNodeWrapper>>,
}