
use crate::foundation::geometry::IntOffset;

pub(crate) trait LayoutNodeWrapper: Placeable + Debug + Measurable {
    fn attach(&mut self, layout_node: Weak<RefCell<LayoutNode>>);
    fn layout_node(&self) -> Weak<RefCell<LayoutNode>>;
    fn on_initialize(&self) {}
    fn on_place(&self) {}

    fn perform_measure(&mut self, _block: &dyn FnOnce()-> Box<dyn Placeable>) {

    }
}

#[derive(Debug)]
pub(crate) struct LayoutNodeWrapperImpl {
    placeable_impl: PlaceableImpl,
    measure_result: MeasureResult,
    wrapped_by: Option<Box<dyn LayoutNodeWrapper>>,
    layout_node: MaybeUninit<Weak<RefCell<LayoutNode>>>,
}