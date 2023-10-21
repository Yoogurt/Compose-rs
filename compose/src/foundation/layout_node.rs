#[derive(Debug)]
pub struct LayoutNode {
    children: Vec<Rc<RefCell<LayoutNode>>>,
    modifier: Modifier,
    measure_policy: Option<MeasurePolicyDelegate>,
    parent_data: Option<Box<dyn ParentData>>,
    measure_result: MeasureResult,
    inner_placeable: InnerPlaceable,
    inner_layout_node: LayoutNodeWrapper,
    outer_placeable: OuterPlaceable,
    outer_layout_node: LayoutNodeWrapper,
    layout_state: LayoutState,
}