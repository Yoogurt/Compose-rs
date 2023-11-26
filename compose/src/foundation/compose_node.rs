use crate::foundation::composer::Composer;
use crate::foundation::layout_node::LayoutNode;

pub(crate) fn ComposeNode(update: impl Fn(&mut LayoutNode) + 'static, mut content: impl FnMut()) {
    Composer::start_node();
    let node = if Composer::inserting() {
        Composer::create_node(move |node| {
            update(&mut node.clone().borrow_mut());
        })
    } else {
        Composer::use_node()
    };
    content();
    Composer::end_node();
}
