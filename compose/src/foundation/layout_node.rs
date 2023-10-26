use std::{cell::RefCell, rc::{Rc, Weak}};

use super::{
    layout_result::PlaceableImpl, layout_state::LayoutState,
    look_ahead_pass_delegate::LookaheadPassDelegate, node_chain::NodeChain,
};

#[derive(Debug)]
pub(crate) struct LayoutNode {
    pub(crate) node_chain: NodeChain,
    pub(crate) layout_node_layout_delegate: Rc<RefCell<LayoutNodeLayoutDelegate>>,
    pub(crate) usage_by_parent: UsageByParent,
}

#[derive(Debug)]
pub(crate) struct MeasurePassDelegate {
    pub(crate) placeable_impl: PlaceableImpl,
    pub(crate) parent: Weak<RefCell<LayoutNodeLayoutDelegate>>,
    pub(crate) remeasure_pending: bool,
}

#[derive(Debug)]
pub(crate) struct LayoutNodeLayoutDelegate {
    pub(crate) measure_pass_delegate: Rc<RefCell<MeasurePassDelegate>>,
    pub(crate) lookahead_pass_delegate: Rc<RefCell<LookaheadPassDelegate>>,
    pub(crate) layout_state: LayoutState,
    pub(crate) children: Vec<Rc<RefCell<LayoutNode>>>,
}

#[derive(Debug)]
pub(crate) enum UsageByParent {
    NotUsed,
    InMeasureBlock,
    InLayoutBlock,
}
