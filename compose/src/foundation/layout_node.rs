#[derive(Debug)]
pub struct LayoutNode {
    usage_by_parent: UsageByParent,
    modifier: Modifier,
    parent_data: Option<Box<dyn ParentData>>,
    measure_result: MeasureResult,
    inner_placeable: Rc<RefCell<InnerPlaceable>>,
    outer_measurable_placeable: OuterMeasurePlaceable,
    outer_layout_node: Rc<RefCell<dyn LayoutNodeWrapper>>,
    layout_state: LayoutState,
}

#[derive(Debug)]
pub(crate) enum UsageByParent {
    NotUsed,
    InMeasureBlock,
    InLayoutBlock
}