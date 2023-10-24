#[derive(Debug)]
pub(crate) struct LayoutNode {
    node_chain: NodeChain,
    layout_node_layout_delegate: Rc<RefCell<LayoutNodeLayoutDelegate>>,
    usage_by_parent: UsageByParent,
    layout_state: LayoutState,
}

#[derive(Debug)]
pub(crate) struct LayoutNodeLayoutDelegate {
    children: Vec<Rc<RefCell<LayoutNode>>>,
}

#[derive(Debug)]
pub(crate) enum UsageByParent {
    NotUsed,
    InMeasureBlock,
    InLayoutBlock
}