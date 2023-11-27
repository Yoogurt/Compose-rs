use crate::foundation::composer::Composer;
use crate::foundation::updater::Updater;

pub(crate) fn ComposeNode(update: impl FnOnce(&mut Updater) + 'static, mut content: impl FnMut()) {
    Composer::start_node();
    let node = if Composer::inserting() {
        Composer::create_node()
    } else {
        Composer::use_node()
    };
    Composer::static_dispatch_mut(move |composer| {
        update(&mut Updater::new(composer));
    });

    content();
    Composer::end_node();
}
