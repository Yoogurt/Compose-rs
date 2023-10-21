use super::{Constraint, OuterPlaceable, LayoutNode, LayoutState, Measurable, Placeable};

impl OuterPlaceable {
    pub(crate) fn new() -> OuterPlaceable {
        OuterPlaceable {
            layout_node: Box::new(LayoutNode::new())
        }
    }

    pub(crate) fn remeasure(&mut self,
                            current_state: LayoutState,
                            constraint: &Constraint,
                            layout_state_changer: impl FnMut(LayoutState)) -> bool {
        false
    }
}

impl Measurable for OuterPlaceable {
    fn measure(&mut self, constraint: &Constraint) -> Placeable {
        todo!()
    }
}