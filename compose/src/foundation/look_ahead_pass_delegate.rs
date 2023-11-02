use auto_delegate::Delegate;
use crate::foundation::constraint::Constraint;
use crate::foundation::measurable::Measurable;
use crate::foundation::placeable::Placeable;
use crate::foundation::placeable_impl::PlaceableImpl;
use crate::foundation::remeasurable::Remeasurable;


#[derive(Delegate, Debug)]
pub(crate) struct LookaheadPassDelegate {
    #[to(Placeable, Measured)]
    pub(crate) placeable_impl: PlaceableImpl,
}

impl LookaheadPassDelegate {
    pub(crate) fn new() -> Self {
        LookaheadPassDelegate {
            placeable_impl: PlaceableImpl::new(),
        }
    }
}

impl Remeasurable for LookaheadPassDelegate {
    fn remeasure(&mut self, _constraint: &Constraint) -> bool {
        todo!()
    }
}

impl Measurable for LookaheadPassDelegate {
    fn measure(&mut self, _constraint: &Constraint) -> &mut dyn Placeable {
        &mut self.placeable_impl
    }
}
