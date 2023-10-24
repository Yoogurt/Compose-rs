use std::mem::MaybeUninit;

#[derive(Debug)]
pub(crate) struct OuterCoordinator {
    layout_node_wrapper: LayoutNodeWrapperImpl,
    layout_node: MaybeUninit<Rc<RefCell<dyn LayoutNodeWrapper>>>,
}