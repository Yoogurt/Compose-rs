use auto_delegate::Delegate;

use super::layout_result::PlaceableImpl;

#[derive(Delegate, Debug)]
pub(crate) struct LookaheadPassDelegate {
    #[to(Placeable,Measured)]
    pub(crate) placeable_impl: PlaceableImpl,
}