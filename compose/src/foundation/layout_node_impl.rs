use std::cell::{Ref, RefCell, RefMut};

use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

use super::canvas::Canvas;
use super::constraint::Constraint;
use super::layout_node::{LayoutNode, UsageByParent, LayoutNodeLayoutDelegate, MeasurePassDelegate};
use super::layout_result::{PlaceableImpl, Placeable};
use super::layout_state::LayoutState;
use super::look_ahead_capable_placeable::LayoutNodeWrapper;
use super::look_ahead_pass_delegate::LookaheadPassDelegate;
use super::measurable::{MultiChildrenMeasurePolicy, Measurable};
use super::measure_result::MeasureResult;
use super::modifier::Modifier;
use super::node_chain::NodeChain;
use super::remeasurable::Remeasurable;

impl Deref for LayoutNode {
    type Target = NodeChain;

    fn deref(&self) -> &Self::Target {
        &self.node_chain
    }
}

impl DerefMut for LayoutNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node_chain
    }
}

impl LayoutNode {
    pub(crate) fn new() -> Rc<RefCell<Self>> {
        let node = LayoutNode {
            node_chain: NodeChain::new(),
            layout_node_layout_delegate: LayoutNodeLayoutDelegate::new(),
            usage_by_parent: UsageByParent::NotUsed,
        };

        let node = Rc::new(RefCell::new(node));
        {
            let layout_node_layout_delegate = node.borrow().layout_node_layout_delegate.clone();

            let mut node_mut = node.borrow_mut();
            node_mut.inner_placeable.borrow_mut().attach(layout_node_layout_delegate);
            let inner_layout_node_wrapper = node_mut.inner_placeable.clone();
            node_mut.outer_measurable_placeable.attach(inner_layout_node_wrapper);
        }

        node
    }

    pub fn set_modifier(&mut self, modifier: Modifier) {
        if self.modifier == modifier {
            return;
        }

        self.modifier = modifier;

        let _outer_wrapper = self.modifier.fold_out::<Rc<RefCell<dyn LayoutNodeWrapper>>>(self.inner_placeable.clone(), &mut |_modifier, to_wrap| {
            let wrapper = to_wrap;

            wrapper
        });
    }

    pub fn set_measure_policy(&self,
                              measure_policy: MultiChildrenMeasurePolicy) {
        self.inner_placeable.borrow_mut().measure_policy = measure_policy;
    }

    fn layout(width: usize, height: usize) -> MeasureResult {
        MeasureResult {
            width,
            height,
        }
    }

    pub(crate) fn adopt_child(&self, _child: Rc<RefCell<LayoutNode>>) {
        // self.inner_placeable.borrow_mut().adopt_child(child);
    }

    pub fn remeasure(&self) -> Rc<RefCell<dyn Remeasurable>> {
        self.layout_node_layout_delegate.borrow().measure_pass_delegate.clone()
    }

    fn draw(_canvas: &dyn Canvas) {}
}

impl Remeasurable for MeasurePassDelegate {
    fn remeasure(&mut self, _constraint: &Constraint) -> bool {
        // let mut previous_size: IntSize;
        // let new_size = {
        //     let parent = self.parent.upgrade();
        //         if parent.is_none() {
        //             panic!("parent node was in used or not exists")
        //         }
        //
        //     let mut inner_layout_node = unsafe { parent.unwrap().borrow_mut() };
        //     previous_size = inner_layout_node.get_measured_size();
        //
        //     inner_layout_node.measure(constraint);
        //     inner_layout_node.get_measured_size()
        // };
        // let size_changed = previous_size != new_size
        //     || self.get_width() != new_size.width() || self.get_height() != new_size.height();
        //
        // self.set_measured_size(new_size);
        // size_changed
        todo!()
    }
}

impl MeasurePassDelegate {
    fn new() -> Self {
        MeasurePassDelegate {
            placeable_impl: PlaceableImpl::new(),
            parent: Weak::new(),
        }
    }

    pub(crate) fn attach(&mut self, parent: Weak<RefCell<LayoutNodeLayoutDelegate>>) {
        self.parent = parent;
    }
}

impl LayoutNodeLayoutDelegate {
    pub(crate) fn new() -> Rc<RefCell<Self>> {
        let result = Rc::new(RefCell::new(LayoutNodeLayoutDelegate {
            measure_pass_delegate: Rc::new(RefCell::new(MeasurePassDelegate::new())),
            lookahead_pass_delegate: Rc::new(RefCell::new(LookaheadPassDelegate::new())),
            layout_state: LayoutState::Ready,
            children: vec![],
        }));

        result.borrow().measure_pass_delegate.borrow_mut().attach(Rc::downgrade(&result));
        result
    }

    pub(crate) fn as_measurable(&self) -> Ref<dyn Measurable> {
        self.measure_pass_delegate.borrow()
    }

    pub(crate) fn as_measurable_mut(&self) -> RefMut<dyn Measurable> {
        self.measure_pass_delegate.borrow_mut()
    }
}

impl Measurable for MeasurePassDelegate {
    fn measure(&mut self, constraint: &Constraint) -> &mut dyn Placeable {
        let parent =self.parent.upgrade();
        if parent.is_none() {
            panic!("unable to parent")
        }
        parent.unwrap().borrow().lookahead_pass_delegate.borrow_mut().measure(constraint);

        self.remeasure(constraint);
        &mut self.placeable_impl
    }
}