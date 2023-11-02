use super::{look_ahead_pass_delegate::LookaheadPassDelegate, placeable::{PlaceableImpl, Placeable}, remeasurable::Remeasurable, measurable::Measurable, constraint::Constraint};

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
