use compose_macro::Composable;
use crate::foundation::composer::Composer;
use crate::foundation::updater::Updater;
use crate as compose;

pub(crate) fn ComposeNode(update: impl FnOnce(&mut Updater) + 'static, mut content: impl FnMut()) {
    Composer::start_node();
    if Composer::inserting() {
        Composer::create_node();
    } else {
        Composer::use_node();
    }
    Composer::static_dispatch_mut(move |composer| {
        update(&mut Updater::new(composer));
    });

    content();
    Composer::end_node();
}
