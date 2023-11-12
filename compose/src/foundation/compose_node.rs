use std::ops::DerefMut;

use crate::foundation::composer::Composer;
use crate::foundation::layout_node::LayoutNode;

pub(crate) fn ComposeNode(mut update: impl FnOnce(&mut LayoutNode) + 'static, mut content: impl FnMut()) {
    Composer::start_node();
    let node = if Composer::inserting() {
        Composer::create_node()
    } else {
        Composer::use_node()
    };
    if Composer::inserting() {
        Composer::record_fix_up(Box::new(move || {
            update(node.clone().borrow_mut().deref_mut());
        }));
    }
    content();
    Composer::end_node();
}
