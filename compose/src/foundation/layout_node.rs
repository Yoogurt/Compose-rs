use std::{cell::RefCell, rc::Rc};
use auto_delegate::Delegate;
use crate::foundation::constraint::Constraint;
use crate::foundation::modifier_container::ModifierContainer;

use super::{
    placeable::PlaceableImpl, layout_state::LayoutState,
    look_ahead_pass_delegate::LookaheadPassDelegate, node_chain::NodeChain, geometry::IntOffset,
};

#[derive(Debug)]
pub(crate) struct LayoutNode {
    pub(crate) modifier_container: Rc<RefCell<ModifierContainer>>,
    pub(crate) node_chain: Rc<RefCell<NodeChain>>,
    pub(crate) children: Rc<RefCell<Vec<Rc<RefCell<LayoutNode>>>>>,
    pub(crate) layout_node_layout_delegate: Rc<RefCell<LayoutNodeLayoutDelegate>>,
    pub(crate) usage_by_parent: UsageByParent,
    pub(crate) layout_state: LayoutState,
}

#[derive(Debug, Delegate)]
pub(crate) struct MeasurePassDelegate {
    #[to(Placeable, Measured)]
    pub(crate) placeable_impl: PlaceableImpl,
    pub(crate) nodes: Option<Rc<RefCell<NodeChain>>>,
    pub(crate) remeasure_pending: bool,
    pub(crate) measure_pending: bool,
    pub(crate) layout_pending: bool,
    pub(crate) layout_state: LayoutState,
    pub(crate) measured_by_parent: UsageByParent,
    pub(crate) last_position: IntOffset,
    pub(crate) last_z_index: f32,
    pub(crate) z_index: f32,
    pub(crate) place_once: bool,
    pub(crate) is_placed: bool,
}

#[derive(Debug)]
pub(crate) struct LayoutNodeLayoutDelegate {
    pub(crate) last_constraints : Option<Constraint>,
    pub(crate) nodes: Option<Rc<RefCell<NodeChain>>>,
    pub(crate) modifier_container: Rc<RefCell<ModifierContainer>>,
    pub(crate) measure_pass_delegate: Rc<RefCell<MeasurePassDelegate>>,
    pub(crate) lookahead_pass_delegate: Rc<RefCell<LookaheadPassDelegate>>,
    pub(crate) measure_pending: bool,
    pub(crate) layout_pending: bool,
}

#[derive(Debug, PartialEq)]
pub(crate) enum UsageByParent {
    NotUsed,
    InMeasureBlock,
    InLayoutBlock,
}
