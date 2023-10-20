use crate::foundation::{LayoutNode, LayoutReceiver};

impl LayoutReceiver {
    pub(crate) fn new(layout_node: &mut LayoutNode) {
        return  LayoutReceiver {
            layout_node
        }
    }

    pub fn layout(width: usize, height: usize) {

    }
}