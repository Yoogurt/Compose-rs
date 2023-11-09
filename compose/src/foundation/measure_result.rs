use std::fmt::{Debug, Formatter};
use auto_delegate::delegate;
use crate::foundation::geometry::IntSize;
use crate::foundation::measure_scope::MeasureScope;
use crate::foundation::placement_scope::PlacementScope;
use crate::foundation::placement_scope_impl::PlacementScopeImpl;

#[derive(Default)]
pub struct MeasureResult {
    pub(crate) width: usize,
    pub(crate) height: usize,

    placement_block: Option<Box<dyn FnOnce(&dyn PlacementScope)>>,
}

impl Debug for MeasureResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MeasureResult")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

impl PartialEq for MeasureResult {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height != other.height && match (&self.placement_block, &other.placement_block) {
            (None, None) => {
                true
            }
            (Some(left_placement_block), Some(right_placement_block)) => {
                std::ptr::eq(left_placement_block, right_placement_block)
            }
            _ => {
                false
            }
        }
    }
}

#[delegate]
pub trait MeasureResultProvider {
    fn set_measured_result(&mut self, measure_result: MeasureResult);
    fn get_measured_result(&mut self) -> Option<MeasureResult>;
    fn has_measure_result(&self) -> bool;
}

impl MeasureResult {
    pub(crate) fn new(size: IntSize, placement_block: Option<Box<dyn FnOnce(&dyn PlacementScope)>>) -> Self {
        MeasureResult {
            width: size.width(),
            height: size.height(),
            placement_block,
        }
    }

    pub(crate) fn place_children(&mut self, measure_scope: &dyn MeasureScope) {
        let place_action = self.placement_block.take();
        if let Some(place_action) = place_action {
            let placement_scope = PlacementScopeImpl::new(self.width, self.height, measure_scope);
            place_action(&placement_scope);
        }
    }

    pub(crate) fn as_int_size(&self) -> IntSize {
        IntSize::new(self.width, self.height)
    }
}