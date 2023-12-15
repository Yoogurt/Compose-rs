use crate::foundation::node_coordinator::NodeCoordinator;
use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;
use crate::foundation::geometry::Offset;
use crate::foundation::node_coordinator_impl::NodeCoordinatorImpl;
use crate::foundation::ui::hit_test_result::HitTestResult;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;
use crate::foundation::layout_node::LayoutNode;

#[derive(Debug)]
pub(crate) struct LayoutNodeHitTestDelegate {
    pub(crate) layout_node: Weak<RefCell<LayoutNode>>,
}

impl LayoutNodeHitTestDelegate {
    pub(crate) fn new() -> Rc<RefCell<Self>> {
        Self {
            layout_node: Default::default(),
        }.wrap_with_rc_refcell()
    }

    pub(crate) fn attach(&mut self, layout_node: &Rc<RefCell<LayoutNode>>) {
        self.layout_node = Rc::downgrade(layout_node);
    }

    fn get_outer_coordinator(&self) -> Rc<RefCell<dyn NodeCoordinator>> {
        self.layout_node.upgrade().unwrap().borrow().get_outer_coordinator()
    }

    pub(crate) fn hit_test(&self, pointer_position: Offset<f32>, hit_test_result: &mut HitTestResult, is_touch_event: bool, is_in_layer: bool) {
        let _outer_coordinator = self.get_outer_coordinator();
        let outer_coordinator = _outer_coordinator.borrow();
        let position_in_wrapped = outer_coordinator.from_parent_position(pointer_position);
        outer_coordinator.hit_test(&NodeCoordinatorImpl::PointerInputSource, position_in_wrapped, hit_test_result, is_touch_event, is_in_layer);
    }
}