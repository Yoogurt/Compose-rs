#[derive(Debug)]
pub(crate) struct LayoutNode {
    node_chain: NodeChain,
    layout_node_layout_delegate: Rc<RefCell<LayoutNodeLayoutDelegate>>,
    usage_by_parent: UsageByParent,
}

#[derive(Debug)]
pub(crate) struct MeasurePassDelegate {
    placeable_impl: PlaceableImpl,
    parent: Weak<RefCell<LayoutNodeLayoutDelegate>>,
}

#[derive(Debug)]
pub(crate) struct LayoutNodeLayoutDelegate {
    pub(crate) measure_pass_delegate: Rc<RefCell<MeasurePassDelegate>>,
        pub(crate)  lookahead_pass_delegate: Rc<RefCell<LookaheadPassDelegate>>,
    layout_state: LayoutState,
    children: Vec<Rc<RefCell<LayoutNode>>>,
}

#[derive(Debug)]
pub(crate) enum UsageByParent {
    NotUsed,
    InMeasureBlock,
    InLayoutBlock
}