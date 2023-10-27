use std::cell::{Ref, RefCell, RefMut};
use std::mem::MaybeUninit;

use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;
use crate::widgets::layout::layout;

use super::canvas::Canvas;
use super::constraint::{Constraint, self};
use super::geometry::IntSize;
use super::layout_node::{LayoutNode, UsageByParent, LayoutNodeLayoutDelegate, MeasurePassDelegate};
use super::layout_result::{PlaceableImpl, Placeable};
use super::layout_state::LayoutState;
use super::look_ahead_capable_placeable::NodeCoordinator;
use super::look_ahead_pass_delegate::LookaheadPassDelegate;
use super::measurable::{MultiChildrenMeasurePolicy, Measurable};
use super::measure_result::MeasureResult;
use super::modifier::Modifier;
use super::node_chain::NodeChain;
use super::remeasurable::{Remeasurable, StatefulRemeasurable};

impl LayoutNode {
    pub(crate) fn new() -> Rc<RefCell<Self>> {
        let node = LayoutNode {
            node_chain: NodeChain::new(),
            children: vec![],
            layout_node_layout_delegate: LayoutNodeLayoutDelegate::new(),
            usage_by_parent: UsageByParent::NotUsed,
        };

        let node = node.wrap_with_rc_refcell();
        {
            let layout_node_layout_delegate = node.borrow().layout_node_layout_delegate.clone();
            let mut node_mut = node.borrow_mut();

            let node_chain = node_mut.node_chain.clone();
            node_chain.borrow_mut().attach(Rc::downgrade(&node));

            node_mut.layout_node_layout_delegate.borrow_mut().attach(node_chain);
        }

        node
    }

    pub(crate) fn get_measure_pass_delegate(&self) -> Rc<RefCell<MeasurePassDelegate>> {
        self.layout_node_layout_delegate.borrow().measure_pass_delegate.clone()
    }

    pub(crate) fn for_each_child<F>(&self, f: F) where F: FnMut(&Rc<RefCell<LayoutNode>>) {
        self.children.iter().for_each(f);
    }

    pub fn set_modifier(&mut self, modifier: Modifier) {
        // if self.modifier == modifier {
        //     return;
        // }
        //
        // self.modifier = modifier;
        //
        // let _outer_wrapper = self.modifier.fold_out::<Rc<RefCell<dyn LayoutNodeWrapper>>>(self.inner_placeable.clone(), &mut |_modifier, to_wrap| {
        //     let wrapper = to_wrap;
        //
        //     wrapper
        // });
    }

    pub fn set_measure_policy(&self,
                              measure_policy: MultiChildrenMeasurePolicy) {
        self.node_chain.borrow().inner_coordinator.borrow_mut().set_measure_policy(measure_policy);
    }

    fn layout(width: usize, height: usize) -> MeasureResult {
        MeasureResult {
            width,
            height,
        }
    }

    pub(crate) fn adopt_child(&mut self, child: Rc<RefCell<LayoutNode>>) {
        self.children.push(child);
    }

    pub fn as_remeasurable(&self) -> Rc<RefCell<dyn StatefulRemeasurable>> {
        self.layout_node_layout_delegate.borrow().measure_pass_delegate.clone()
    }

    fn draw(_canvas: &dyn Canvas) {}
}

impl Remeasurable for MeasurePassDelegate {
    fn remeasure(&mut self, constraint: &Constraint) -> bool {
        let mut previous_size: IntSize = {
            let outer_node_ref = unsafe {
                self.nodes.assume_init_ref().borrow()
            };
            let mut outer_coodinator = outer_node_ref.outer_coordinator.borrow_mut();
            outer_coodinator.get_measured_size()
        };

        self.perform_measure(constraint);

        let new_size = {
            let outer_node_ref = unsafe {
                self.nodes.assume_init_ref().borrow()
            };
            let mut outer_coodinator = outer_node_ref.outer_coordinator.borrow_mut();
            outer_coodinator.get_measured_size()
        };

        let size_changed = previous_size != new_size
            || self.get_width() != new_size.width() || self.get_height() != new_size.height();

        self.set_measured_size(new_size);
        size_changed
    }
}

impl StatefulRemeasurable for MeasurePassDelegate {
    fn mark_remeasure_pending(&mut self) {
        self.remeasure_pending = true;
    }
}

impl MeasurePassDelegate {
    fn new() -> Self {
        MeasurePassDelegate {
            placeable_impl: PlaceableImpl::new(),
            nodes: MaybeUninit::uninit(),
            remeasure_pending: false,
            measure_pending: false,
            layout_pending: false,
            layout_state: LayoutState::Idle,
            measured_by_parent: UsageByParent::NotUsed,
        }
    }

    pub(crate) fn set_measured_by_parent(&mut self, measured_by_parent: UsageByParent) {
        self.measured_by_parent = measured_by_parent;
    }

    pub(crate) fn attach(&mut self, node_chain: Rc<RefCell<NodeChain>>) {
        self.nodes = MaybeUninit::new(node_chain);
    }

    pub(crate) fn mark_measure_pending(&mut self) {
        self.measure_pending = true;
    }

    pub(crate) fn mark_layout_pending(&mut self) {
        self.layout_pending = true;
    }

    pub(crate) fn perform_measure(&mut self, constraint: &Constraint) {
        if self.layout_state != LayoutState::Idle {
            panic!("layout state is not idle before measure starts")
        }
        self.layout_state = LayoutState::Measuring;
        self.measure_pending = false;

        unsafe { self.nodes.assume_init_ref() }.borrow_mut().outer_coordinator.borrow_mut().measure(constraint);

        if self.layout_state == LayoutState::Measuring {
            self.mark_layout_pending();
        }
    }
}

impl LayoutNodeLayoutDelegate {
    pub(crate) fn new() -> Rc<RefCell<Self>> {
        LayoutNodeLayoutDelegate {
            last_constraints: None,
            nodes: MaybeUninit::uninit(),
            measure_pass_delegate: MeasurePassDelegate::new().wrap_with_rc_refcell(),
            lookahead_pass_delegate: LookaheadPassDelegate::new().wrap_with_rc_refcell(),
            layout_state: LayoutState::Idle,
            measure_pending: false,
            layout_pending: false,
        }.wrap_with_rc_refcell()
    }

    pub(crate) fn attach(&mut self, node_chain: Rc<RefCell<NodeChain>>) {
        self.nodes = MaybeUninit::new(node_chain.clone());
        self.measure_pass_delegate.borrow_mut().attach(node_chain);
    }

    pub(crate) fn as_measurable(&self) -> Ref<dyn Measurable> {
        self.measure_pass_delegate.borrow()
    }

    pub(crate) fn as_measurable_mut(&self) -> RefMut<dyn Measurable> {
        self.measure_pass_delegate.borrow_mut()
    }

    pub fn remeasure(&mut self, mut constraint: Option<Constraint>) -> bool {
        if constraint.is_none() {
            constraint = self.last_constraints;
        }

        match constraint {
            Some(constraint) => {
                self.measure_pass_delegate.clone().borrow_mut().remeasure(&constraint)
            }
            None => {
                false
            }
        }
    }
}

impl Measurable for MeasurePassDelegate {
    fn measure(&mut self, constraint: &Constraint) -> &mut dyn Placeable {
        // let parent = self.parent.upgrade();
        // if parent.is_none() {
        //     panic!("unable to parent")
        // }
        // parent.unwrap().borrow().lookahead_pass_delegate.borrow_mut().measure(constraint);
        //
        <Self as Remeasurable>::remeasure(self, constraint);
        &mut self.placeable_impl
    }
}