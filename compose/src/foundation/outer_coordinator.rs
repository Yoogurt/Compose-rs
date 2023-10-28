use std::{mem::MaybeUninit, cell::RefCell, rc::Rc};
use auto_delegate::Delegate;

use super::look_ahead_capable_placeable::{NodeCoordinatorImpl, NodeCoordinator};

#[derive( Delegate)]
pub(crate) struct OuterCoordinator {
    #[to(Placeable, Measured)]
    pub(crate) node_coordinator_impl: NodeCoordinatorImpl,
    pub(crate) layout_node: MaybeUninit<Rc<RefCell<dyn NodeCoordinator>>>,
}