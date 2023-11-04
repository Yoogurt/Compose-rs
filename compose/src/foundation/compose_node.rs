use crate::foundation::composer::Composer;
use crate::foundation::layout_node::LayoutNode;
use std::ops::DerefMut;

pub(crate) fn ComposeNode(mut update: impl FnOnce(&mut LayoutNode), mut content: impl FnMut()) {
    Composer::start_node();
    let node = if Composer::inserting() {
        Composer::create_node()
    } else {
        Composer::use_node()
    };
    update(node.borrow_mut().deref_mut());
    content();
    Composer::end_node();
}
