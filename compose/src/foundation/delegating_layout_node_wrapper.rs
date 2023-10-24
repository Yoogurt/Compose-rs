pub(crate) trait DelegatingLayoutNodeWrapper: LayoutNodeWrapper {
    fn set_modifier_to(&mut self, modifier: Modifier);
}

#[derive(Debug)]
pub struct DelegatingLayoutNodeWrapperImpl {
    wrapped: Rc<RefCell<dyn LayoutNodeWrapper>>,
    modifier: Modifier,

    layout_node_wrapper_impl: LayoutNodeWrapperImpl
}