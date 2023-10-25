use auto_delegate::Delegate;

#[derive(Delegate, Debug)]
pub(crate) struct LookaheadPassDelegate {
    #[to(Placeable,Measured)]
    placeable_impl: PlaceableImpl,
}