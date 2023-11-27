use crate::foundation::composer_impl::ComposerImpl;
use crate::foundation::layout_node::LayoutNode;

pub(crate) struct Updater<'a> {
    composer: &'a mut ComposerImpl,
}

impl<'a> Updater<'a> {
    pub fn new(composer: &'a mut ComposerImpl) -> Self {
        Self {
            composer
        }
    }

    pub fn set<V: 'static>(&mut self, value: V, block: impl FnOnce(&mut LayoutNode, V) + 'static) {
        if self.composer.inserting {
            self.composer.apply(value, block)
        }
    }
}