use std::rc::Weak;

#[derive(Debug)]
pub(crate) struct InnerCoordinator {
    layout_node_wrapper_impl: LayoutNodeWrapperImpl,
    layout_node_layout_delegate: MaybeUninit<Rc<RefCell<LayoutNodeLayoutDelegate>>>,
    measure_policy: MultiChildrenMeasurePolicy,
}