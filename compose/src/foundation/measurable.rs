use std::cell::RefCell;
use std::rc::Rc;

use crate::foundation::geometry::IntSize;
use crate::foundation::intrinsic_measurable::IntrinsicMeasurable;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;

use super::{
    constraint::Constraints, measure_result::MeasureResult, measure_scope::MeasureScope,
    placeable::Placeable,
};

pub trait Measurable: IntrinsicMeasurable {
    fn measure(&mut self, constraint: &Constraints) -> (IntSize, Rc<RefCell<dyn Placeable>>);

    fn as_placeable(&mut self) -> Rc<RefCell<dyn Placeable>>;
    fn as_measurable_mut(&mut self) -> &mut dyn Measurable;
}

pub type SingleChildMeasurePolicy =
Rc<RefCell<dyn FnMut(&mut dyn MeasureScope, &mut dyn Measurable, &Constraints) -> MeasureResult>>;

#[inline]
pub fn SingleChildMeasurePolicyDelegate(
    delegate: impl FnMut(&mut dyn MeasureScope, &mut dyn Measurable, &Constraints) -> MeasureResult + 'static,
) -> SingleChildMeasurePolicy {
    delegate.wrap_with_rc_refcell()
}

pub type MultiChildrenMeasurePolicy = Rc<RefCell<
    dyn FnMut(&dyn MeasureScope, &mut [&mut dyn Measurable], &Constraints) -> MeasureResult,
>>;

#[inline]
pub fn MultiChildrenMeasurePolicyDelegate(
    delegate: impl FnMut(&dyn MeasureScope, &mut [&mut dyn Measurable], &Constraints) -> MeasureResult + 'static,
) -> MultiChildrenMeasurePolicy {
    delegate.wrap_with_rc_refcell()
}