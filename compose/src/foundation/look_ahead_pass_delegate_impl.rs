
use crate::foundation::{LookaheadPassDelegate, Constraint, Placeable, PlaceableImpl, Measurable};

use super::Remeasurable;


impl LookaheadPassDelegate {
    pub(crate) fn new() -> Self {
        LookaheadPassDelegate {
            placeable_impl: PlaceableImpl::new()
        }
    }
}

impl Remeasurable for LookaheadPassDelegate {
    
}

impl Measurable for LookaheadPassDelegate {
    fn measure(&mut self, _constraint: &Constraint) -> &mut dyn Placeable {
        &mut self.placeable_impl
    }
}