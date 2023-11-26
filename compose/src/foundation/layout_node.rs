use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::{Rc, Weak};
use std::sync::atomic::AtomicU32;
use crate::foundation::compose_node_lifecycle_callback::ComposeNodeLifecycleCallback;

use crate::foundation::geometry::Density;
use crate::foundation::layout_node_container::LayoutNodeContainer;
use crate::foundation::layout_node_draw_delegate::LayoutNodeDrawDelegate;
use crate::foundation::layout_node_layout_delegate::LayoutNodeLayoutDelegate;
use crate::foundation::measure_pass_delegate::MeasurePassDelegate;
use crate::foundation::node::Owner;
use crate::foundation::node_coordinator::NodeCoordinator;
use crate::foundation::usage_by_parent::UsageByParent;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;

use super::{layout_state::LayoutState, node_chain::NodeChain};
use super::canvas::Canvas;
use super::measurable::MultiChildrenMeasurePolicy;
use super::modifier::Modifier;
use super::remeasurable::StatefulRemeasurable;

thread_local! {
    static IDENTIFY: AtomicU32 = AtomicU32::new(0);
}

#[derive(Debug)]
pub(crate) struct LayoutNode {
    pub(crate) layout_node_container: Rc<RefCell<LayoutNodeContainer>>,
    pub(crate) node_chain: Rc<RefCell<NodeChain>>,
    pub(crate) children: Rc<RefCell<Vec<Rc<RefCell<LayoutNode>>>>>,
    pub(crate) layout_node_layout_delegate: Rc<RefCell<LayoutNodeLayoutDelegate>>,
    pub(crate) layout_node_draw_delegate: Rc<RefCell<LayoutNodeDrawDelegate>>,
    pub(crate) usage_by_parent: UsageByParent,
    pub(crate) layout_state: Rc<RefCell<LayoutState>>,

    pub(crate) owner: Option<Weak<RefCell<dyn Owner>>>,
    pub(crate) identify: u32,
}

impl LayoutNode {
    pub(crate) fn new() -> Rc<RefCell<Self>> {
        let node = LayoutNode {
            layout_node_container: LayoutNodeContainer::new().wrap_with_rc_refcell(),
            node_chain: NodeChain::new(),
            children: vec![].wrap_with_rc_refcell(),
            layout_node_layout_delegate: LayoutNodeLayoutDelegate::new(),
            layout_node_draw_delegate: LayoutNodeDrawDelegate::new(),
            usage_by_parent: UsageByParent::NotUsed,
            layout_state: LayoutState::Idle.wrap_with_rc_refcell(),

            owner: None,
            identify: IDENTIFY.with(|identity| identity.fetch_add(1, std::sync::atomic::Ordering::SeqCst)),
        };

        let node = node.wrap_with_rc_refcell();
        {
            let node_ref = node.borrow();
            let identify = node_ref.identify;

            let node_chain = node_ref.node_chain.clone();
            let modifier_container = node_ref.layout_node_container.clone();
            node_chain
                .borrow_mut()
                .attach(identify, &node,
                        &modifier_container,
                        &node_ref.layout_node_layout_delegate.borrow().measure_pass_delegate,
                        &node_ref.node_chain);

            let layout_state = node_ref.layout_state.clone();
            node_ref.layout_node_layout_delegate.borrow_mut().attach(
                identify,
                &node_chain,
                &modifier_container,
                &layout_state,
            );

            node_ref.layout_node_draw_delegate.borrow_mut().attach(node_chain);
        }

        node
    }

    pub fn attach(&mut self, parent: Option<&LayoutNode>, owner: Weak<RefCell<dyn Owner>>) {
        if parent.is_none() {
            self.get_measure_pass_delegate().borrow_mut().is_placed = true;
        }

        self.get_outer_coordinator().borrow_mut().set_wrapped_by(parent.and_then(|parent| Some(
            Rc::downgrade(&parent.get_inner_coordinator())
        )));

        self.owner = Some(owner.clone());
        owner.upgrade().unwrap().borrow().on_attach(self);

        self.for_each_child(|child| {
            child.borrow_mut().attach(Some(self), owner.clone());
        });

        self.layout_node_layout_delegate.borrow().update_parent_data();
    }

    pub fn detach(&mut self) {
        let owner = self.owner.take();
        match owner {
            None => {
                panic!("Cannot detach node that is already detached!")
            }
            Some(owner) => {
                self.for_each_child(|child| {
                    child.borrow_mut().detach();
                });
            }
        }
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

    pub(crate) fn get_inner_coordinator(&self) -> Rc<RefCell<dyn NodeCoordinator>> {
        self.node_chain.borrow().inner_coordinator.clone()
    }

    pub(crate) fn adopt_child(
        this: &Rc<RefCell<LayoutNode>>,
        child: &Rc<RefCell<LayoutNode>>,
        is_root: bool,
    ) {
        this.borrow().children.borrow_mut().push(child.clone());
        if !is_root {
            child
                .borrow()
                .node_chain
                .borrow_mut()
                .set_parent(Some(Rc::downgrade(this)));
        }

        let owner = this.borrow().owner.clone();
        if let Some(owner) = owner {
            child.borrow_mut().attach(Some(&this.borrow()), owner);
        }
    }

    pub(crate) fn set_parent(&self, parent: Option<Weak<RefCell<LayoutNode>>>) {
        self.node_chain.borrow_mut().set_parent(parent)
    }

    pub(crate) fn on_remove_child(&self,
                                  child: &Rc<RefCell<LayoutNode>>,
    ) {
        let mut child = child.borrow_mut();
        if self.owner.is_some() {
            child.detach();
        }

        child.set_parent(None);
        child.get_outer_coordinator().borrow_mut().set_wrapped_by(None);
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

    pub(crate) fn get_density(&self) -> Density {
        Density::new(
            1.0,
            1.0,
        )
    }

    pub(crate) fn get_parent(&self) -> Option<Weak<RefCell<LayoutNode>>> {
        self.node_chain.borrow().parent.clone()
    }

    pub(crate) fn request_remeasure(&self) {}

    pub(crate) fn remove_at(&mut self, index: usize, count: usize) {
        let mut children = self.children.borrow_mut();
        for i in (index + count - 1)..=(index) {
            let child = children.remove(i);
            self.on_remove_child(&child);
        }
    }

    pub(crate) fn remove_all(&mut self) {
        let mut children = self.children.borrow_mut();
        children.iter().rev().for_each(|child| {
            self.on_remove_child(child);
        });

        children.clear();
    }
}

impl ComposeNodeLifecycleCallback for LayoutNode {
    fn on_reuse(&mut self) {}

    fn on_deactivate(&mut self) {}

    fn on_release(&mut self) {}
}
