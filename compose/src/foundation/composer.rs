use std::{cell::RefCell, rc::Rc};

use crate::foundation::composer_inner::ComposerInner;
use crate::foundation::constraint::Constraints;
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::snapshot_value::SnapShotValue;

pub struct Composer {
    pub(crate) inner: RefCell<ComposerInner>,
}

thread_local! {
    pub static COMPOSER : Composer = Composer::default()
}

impl Composer {
    pub(crate) fn attach_root_layout_node(root: Rc<RefCell<LayoutNode>>) -> bool {
        COMPOSER.with(|local_composer| {
            local_composer
                .inner
                .borrow_mut()
                .attach_root_layout_node(root)
        })
    }

    pub fn destroy() {
        COMPOSER.with(|local_composer| local_composer.inner.borrow_mut().destroy())
    }

    pub(crate) fn detach_root_layout_node() {
        COMPOSER.with(|local_composer| {
            local_composer.inner.borrow_mut().detach_root_layout_node();
        })
    }

    pub fn start_group(hash: i64) {
        COMPOSER.with(|local_composer| {
            local_composer.inner.borrow_mut().start_group(hash);
        })
    }

    pub(crate) fn start_root() {
        COMPOSER.with(|local_composer| local_composer.inner.borrow_mut().start_root())
    }

    pub(crate) fn end_root() {
        COMPOSER.with(|local_composer| local_composer.inner.borrow_mut().end_root())
    }

    pub(crate) fn start_node() {
        COMPOSER.with(|local_composer| local_composer.inner.borrow_mut().start_node())
    }

    pub(crate) fn create_node(factory: impl FnOnce(Rc<RefCell<LayoutNode>>) + 'static) -> Rc<RefCell<LayoutNode>> {
        COMPOSER.with(move |local_composer| local_composer.inner.borrow_mut().create_node(Box::new(factory)))
    }

    pub(crate) fn use_node() -> Rc<RefCell<LayoutNode>> {
        COMPOSER.with(|local_composer| local_composer.inner.borrow_mut().use_node())
    }

    pub(crate) fn record_fix_up(fix_up: Box<dyn FnOnce()>) {
        COMPOSER.with(move |local_composer| local_composer.inner.borrow_mut().record_fix_up(fix_up))
    }

    pub(crate) fn record_insert_up_fix_up(insert_up: Box<dyn FnOnce()>) {
        COMPOSER.with(move |local_composer| {
            local_composer
                .inner
                .borrow_mut()
                .record_insert_up_fix_up(insert_up)
        })
    }

    pub(crate) fn record_deferred_change(&mut self, derred_change: Box<dyn FnOnce()>) {
        COMPOSER.with(move |local_composer| {
            local_composer
                .inner
                .borrow_mut()
                .record_deferred_change(derred_change)
        })
    }

    pub(crate) fn cache<R, T>(keys: &R, calculation: impl FnOnce() -> T) -> SnapShotValue<T>
        where T: 'static, R: Sized + PartialEq<R> + 'static {
        COMPOSER.with(move |local_composer| local_composer.inner.borrow_mut().cache(keys, calculation))
    }

    pub fn apply_changes() {
        COMPOSER.with(move |local_composer| local_composer.inner.borrow_mut().apply_changes())
    }

    pub fn apply_deferred_changes() {
        COMPOSER
            .with(move |local_composer| local_composer.inner.borrow_mut().apply_deferred_changes())
    }

    pub(crate) fn end_node() {
        COMPOSER.with(|local_composer| {
            local_composer.inner.borrow_mut().end_node();
        })
    }

    pub(crate) fn inserting() -> bool {
        COMPOSER.with(|local_composer| local_composer.inner.borrow().inserting())
    }

    pub fn end_group(hash: i64) {
        COMPOSER.with(|local_composer| {
            local_composer.inner.borrow_mut().end_group(hash);
        })
    }

    pub fn validate_group() {
        COMPOSER.with(|local_composer| local_composer.inner.borrow_mut().validate_group())
    }

    pub fn debug_print() {
        COMPOSER.with(|local_composer| local_composer.inner.borrow().debug_print())
    }

    pub fn skip_compose() {}

    pub fn skip_to_group() {}
}

impl Default for Composer {
    fn default() -> Self {
        Composer {
            inner: RefCell::new(Default::default()),
        }
    }
}
