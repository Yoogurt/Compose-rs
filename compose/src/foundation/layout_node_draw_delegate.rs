use std::rc::Rc;
use std::cell::RefCell;
use crate::foundation::canvas::Canvas;
use crate::foundation::node_chain::NodeChain;
use crate::foundation::node_coordinator::NodeCoordinator;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;

#[derive(Debug, Default)]
pub struct LayoutNodeDrawDelegate {
    node_chain: Option<Rc<RefCell<NodeChain>>>,
}

impl LayoutNodeDrawDelegate {
    pub(crate) fn new() -> Rc<RefCell<Self>> {
        LayoutNodeDrawDelegate {
            node_chain: None
        }.wrap_with_rc_refcell()
    }

    pub(crate) fn attach(&mut self, node_chain: Rc<RefCell<NodeChain>>) {
        self.node_chain = Some(node_chain);
    }

    pub(crate) fn draw(&mut self, canvas: &mut dyn Canvas) {
        let outer_coordinator = self.node_chain.as_ref().unwrap().borrow().outer_coordinator.clone();
        outer_coordinator.borrow().draw(canvas);
    }
}
