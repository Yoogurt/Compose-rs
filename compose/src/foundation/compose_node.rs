use std::ops::DerefMut;
use crate::foundation::composer::Composer;
use crate::foundation::layout_node::LayoutNode;

pub(crate) fn ComposeNode(mut update: impl FnOnce(&mut LayoutNode)) {
    Composer::start_node();
    let node = if Composer::inserting() {
        Composer::create_node()
    } else {
        todo!()
    };
        update(node.borrow_mut().deref_mut());
    Composer::end_node();
}