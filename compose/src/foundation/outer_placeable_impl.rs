use super::{Constraint, OuterPlaceable, LayoutNode, LayoutState};

impl OuterPlaceable {
    pub(crate) fn new() -> OuterPlaceable {
        OuterPlaceable {
        }
    }

    pub(crate) fn remeasure(&mut self,
                            current_state: LayoutState,
                            constraint: &Constraint,
                            layout_state_changer: impl FnMut(LayoutState)) -> bool {
        false
    }
}