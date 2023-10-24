use std::rc::Weak;

#[derive(Debug)]
pub(crate) struct InnerPlaceable {
    layout_node_wrapper_impl: LayoutNodeWrapperImpl,
    children: Vec<Rc<RefCell<LayoutNode>>>,
    measure_policy: MultiChildrenMeasurePolicy,
}