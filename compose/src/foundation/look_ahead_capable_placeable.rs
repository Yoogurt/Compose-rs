
use std::{rc::Weak, cell::RefCell, mem::MaybeUninit};
use core::fmt::Debug;
use super::{layout_result::{Placeable, PlaceableImpl}, measurable::Measurable, layout_node::LayoutNode, measure_result::MeasureResult};

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
    pub(crate) placeable_impl: PlaceableImpl,
    pub(crate) measure_result: MeasureResult,
    pub(crate) wrapped_by: Option<Box<dyn LayoutNodeWrapper>>,
    pub(crate) layout_node: MaybeUninit<Weak<RefCell<LayoutNode>>>,
}