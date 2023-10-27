#[derive(Default,Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub(crate) enum LayoutState {
    Measuring,
    LookaheadMeasuring,
    LayingOut,
    LookaheadLayingOut,
    #[default]
    Idle,
}