use auto_delegate::Delegate;

use super::placeable::PlaceableImpl;

#[derive(Delegate, Debug)]
pub(crate) struct LookaheadPassDelegate {
    #[to(Placeable,Measured)]
    pub(crate) placeable_impl: PlaceableImpl,
}