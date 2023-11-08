use crate::foundation::layout_node_layout_delegate::LayoutNodeLayoutDelegate;
use crate::foundation::layout_node_draw_delegate::LayoutNodeDrawDelegate;
use crate::foundation::measure_pass_delegate::MeasurePassDelegate;
use crate::foundation::modifier_container::ModifierContainer;
use crate::foundation::usage_by_parent::UsageByParent;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::vec_deque::IterMut;
use std::rc::{Rc, Weak};
use crate::foundation::node_coordinator::NodeCoordinator;

use super::canvas::Canvas;
use super::measurable::MultiChildrenMeasurePolicy;
use super::measure_result::MeasureResult;
use super::modifier::Modifier;
use super::remeasurable::StatefulRemeasurable;
use super::{layout_state::LayoutState, node_chain::NodeChain};

#[derive(Debug)]
pub(crate) struct LayoutNode {
    pub(crate) modifier_container: Rc<RefCell<ModifierContainer>>,
    pub(crate) node_chain: Rc<RefCell<NodeChain>>,
    pub(crate) children: Rc<RefCell<Vec<Rc<RefCell<LayoutNode>>>>>,
    pub(crate) layout_node_layout_delegate: Rc<RefCell<LayoutNodeLayoutDelegate>>,
    pub(crate) layout_node_draw_delegate: Rc<RefCell<LayoutNodeDrawDelegate>>,
    pub(crate) usage_by_parent: UsageByParent,
    pub(crate) layout_state: Rc<RefCell<LayoutState>>,
}

impl LayoutNode {
    pub(crate) fn new() -> Rc<RefCell<Self>> {
        let node = LayoutNode {
            modifier_container: ModifierContainer::new().wrap_with_rc_refcell(),
            node_chain: NodeChain::new(),
            children: vec![].wrap_with_rc_refcell(),
            layout_node_layout_delegate: LayoutNodeLayoutDelegate::new(),
            layout_node_draw_delegate: LayoutNodeDrawDelegate::new(),
            usage_by_parent: UsageByParent::NotUsed,
            layout_state: LayoutState::Idle.wrap_with_rc_refcell(),
        };

        let node = node.wrap_with_rc_refcell();
        {
            let node_mut = node.borrow_mut();

            let node_chain = node_mut.node_chain.clone();
            let modifier_container = node_mut.modifier_container.clone();
            node_chain
                .borrow_mut()
                .attach(&node,
                        &modifier_container,
                &node_mut.layout_node_layout_delegate.borrow().measure_pass_delegate);

            let layout_state = node_mut.layout_state.clone();
            node_mut.layout_node_layout_delegate.borrow_mut().attach(
                &node_chain,
                &modifier_container,
                &layout_state,
            );

            node_mut.layout_node_draw_delegate.borrow_mut().attach(node_chain);
        }

        node
    }

    fn z_comparator(left: &Rc<RefCell<LayoutNode>>, right: &Rc<RefCell<LayoutNode>>) -> Ordering {
        left.borrow().get_measure_pass_delegate().borrow().z_index.partial_cmp(&right.borrow().get_measure_pass_delegate().borrow().z_index).unwrap()
    }

    pub(crate) fn z_sort_children(&self) -> Vec<Rc<RefCell<LayoutNode>>> {
        let mut result = self.children.borrow().clone();
        result.sort_by(Self::z_comparator);
        result
    }

    pub(crate) fn measure_affects_parent(&self) -> bool {
        self.usage_by_parent == UsageByParent::InMeasureBlock
    }

    pub(crate) fn get_layout_state(&self) -> LayoutState {
        *self.layout_state.borrow()
    }

    pub(crate) fn set_layout_state(&mut self, layout_state: LayoutState) {
        *self.layout_state.borrow_mut() = layout_state
    }

    pub(crate) fn get_children(&self) -> Rc<RefCell<Vec<Rc<RefCell<LayoutNode>>>>> {
        self.children.clone()
    }

    pub(crate) fn get_measure_pass_delegate(&self) -> Rc<RefCell<MeasurePassDelegate>> {
        self.layout_node_layout_delegate
            .borrow()
            .measure_pass_delegate
            .clone()
    }

    pub(crate) fn for_each_child<F>(&self, f: F)
        where
            F: FnMut(&Rc<RefCell<LayoutNode>>),
    {
        self.children.borrow().iter().for_each(f);
    }

    pub fn set_measure_policy(&self, measure_policy: MultiChildrenMeasurePolicy) {
        self.node_chain
            .borrow()
            .inner_coordinator
            .borrow_mut()
            .set_measure_policy(measure_policy);
    }

    pub(crate) fn get_outer_coordinator(&self) -> Rc<RefCell<dyn NodeCoordinator>> {
        self.node_chain.borrow().outer_coordinator.clone()
    }

    fn layout(width: usize, height: usize) -> MeasureResult {
        MeasureResult { width, height }
    }

    pub(crate) fn adopt_child(
        self_: &Rc<RefCell<LayoutNode>>,
        child: &Rc<RefCell<LayoutNode>>,
        is_root: bool,
    ) {
        self_.borrow().children.borrow_mut().push(child.clone());
        if !is_root {
            child
                .borrow()
                .node_chain
                .borrow_mut()
                .set_parent(Rc::downgrade(self_));
        }
    }

    pub fn as_remeasurable(&self) -> Rc<RefCell<dyn StatefulRemeasurable>> {
        self.layout_node_layout_delegate
            .borrow()
            .measure_pass_delegate
            .clone()
    }

    pub fn set_modifier(&self, mut modifier: Modifier) {
        self.node_chain.borrow_mut().update_from(modifier);
        self.layout_node_layout_delegate
            .borrow_mut()
            .update_parent_data();
    }

    pub(crate) fn get_parent(&self) -> Weak<RefCell<LayoutNode>> {
        self.node_chain.borrow().parent.clone()
    }

    fn draw(_canvas: &dyn Canvas) {}
}
