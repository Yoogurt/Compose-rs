use std::ops::DerefMut;
use compose_macro::Composable;
use crate as compose;

use crate::foundation::composer::Composer;
use crate::foundation::layout_node::LayoutNode;

#[Composable]
pub(crate) fn ComposeNode(mut update: impl FnOnce(&mut LayoutNode) + 'static, mut content: impl FnMut()) {
    Composer::start_node();
    let node = if Composer::inserting() {
        Composer::create_node(|node| {
            update(&mut node.clone().borrow_mut());
        })
    } else {
        Composer::use_node()
    };
    content();
    Composer::end_node();
}
