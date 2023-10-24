use std::mem::MaybeUninit;

#[derive(Debug)]
pub(crate) struct OuterMeasurePlaceable {
    layout_node_wrapper: LayoutNodeWrapperImpl,
    layout_node: MaybeUninit<Rc<RefCell<dyn LayoutNodeWrapper>>>,
}