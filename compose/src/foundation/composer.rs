use std::{cell::RefCell, rc::Rc};
use std::ops::{Deref, DerefMut};
use crate::foundation::applier::Applier;

use std::any::Any;
use crate::foundation::composer_impl::{ComposerImpl};
use crate::foundation::constraint::Constraints;
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::recompose_scope_impl::RecomposeScope;
use crate::foundation::remember_manager::RememberManager;
use crate::foundation::snapshot_value::SnapShotValue;
use crate::foundation::composer_impl::ApplierInType;

pub trait ScopeUpdateScope {
    fn update_scope(&mut self, block: Box<dyn FnMut()>);
}

pub trait ScopeUpdateScopeHelper {
    fn update_scope(&mut self, block: impl FnMut() + 'static);
}

impl<T> ScopeUpdateScopeHelper for T where T: DerefMut<Target=dyn ScopeUpdateScope> {
    fn update_scope(&mut self, block: impl FnMut() + 'static) {
        self.deref_mut().update_scope(Box::new(block))
    }
}

#[derive(Default)]
pub struct Composer {
    pub(crate) compose_impl: Option<RefCell<ComposerImpl>>,
}

thread_local! {
    pub static COMPOSER: RefCell<Composer> = RefCell::new(Composer::default())
}

impl Composer {
    pub(crate) fn attach_root_layout_node(root: Rc<RefCell<LayoutNode>>) -> bool {
        COMPOSER.with(move |local_composer| {
            let mut local_composer = local_composer.borrow_mut();
            let mut compose_impl = RefCell::new(ComposerImpl::new(root.clone()));
            let result = compose_impl.borrow_mut().attach_root_layout_node(root);
            local_composer.compose_impl = Some(compose_impl);

            result
        })
    }

    pub(crate) fn static_dispatch<R>(action: impl FnOnce(&ComposerImpl) -> R) -> R {
        COMPOSER.with(|local_composer| action(local_composer.borrow().compose_impl.as_ref().unwrap().borrow().deref()))
    }

    pub(crate) fn static_dispatch_mut<R>(action: impl FnOnce(&mut ComposerImpl) -> R) -> R {
        COMPOSER.with(|local_composer| action(local_composer.borrow().compose_impl.as_ref().unwrap().borrow_mut().deref_mut()))
    }

    pub fn destroy() {
        Self::static_dispatch_mut(|composer| composer.destroy());
        COMPOSER.with(|local_composer| {
            local_composer.borrow_mut().compose_impl = None;
        })
    }

    pub(crate) fn detach_root_layout_node() {
        Self::static_dispatch_mut(|composer| composer.detach_root_layout_node())
    }

    pub fn start_group(hash: u64) {
        Self::static_dispatch_mut(move |composer| composer.start_group(hash));
    }

    pub(crate) fn start_node() {
        Self::static_dispatch_mut(move |composer| composer.start_node());
    }

    pub(crate) fn create_node() {
        Self::static_dispatch_mut(move |composer| composer.create_node())
    }

    pub(crate) fn use_node() -> Rc<RefCell<LayoutNode>> {
        Self::static_dispatch_mut(move |composer| composer.use_node())
    }

    fn record_fix_up(fix_up: impl FnOnce(&mut dyn Applier<Rc<RefCell<LayoutNode>>>, &mut dyn RememberManager) + 'static) {
        Self::static_dispatch_mut(move |composer| composer.record_fix_up(fix_up))
    }

    fn record_insert_up_fix_up(insert_up: impl FnOnce(&mut dyn Applier<ApplierInType>, &mut dyn RememberManager) + 'static) {
        Self::static_dispatch_mut(move |composer| composer.record_insert_up_fix_up(insert_up))
    }

    pub(crate) fn record_deferred_change(&mut self, derred_change: impl FnOnce(&mut dyn Applier<ApplierInType>, &mut dyn RememberManager) + 'static) {
        Self::static_dispatch_mut(move |composer| composer.record_deferred_change(derred_change))
    }

    pub(crate) fn cache<R, T>(keys: &R, calculation: impl FnOnce() -> T) -> SnapShotValue<T>
        where T: 'static, R: Sized + PartialEq<R> + 'static {
        Self::static_dispatch_mut(move |composer| composer.cache(keys, calculation))
    }

    pub fn apply_changes() {
        Self::static_dispatch_mut(move |composer| composer.apply_changes())
    }

    pub fn apply_deferred_changes() {
        Self::static_dispatch_mut(move |composer| composer.apply_deferred_changes())
    }

    pub(crate) fn end_node() {
        Self::static_dispatch_mut(move |composer| composer.end_node())
    }

    pub(crate) fn inserting() -> bool {
        Self::static_dispatch(move |composer| composer.inserting())
    }

    pub fn end_group(hash: u64) {
        Self::static_dispatch_mut(move |composer| composer.end_group(hash))
    }

    pub fn validate_group() {
        Self::static_dispatch_mut(move |composer| composer.validate_group())
    }

    pub fn debug_print() {
        Self::static_dispatch(move |composer| composer.debug_print())
    }

    pub fn recompose_scope() -> Option<Rc<RefCell<dyn RecomposeScope>>> {
        Self::static_dispatch_mut(move |composer| composer.recompose_scope())
    }

    pub fn do_set_content(content: impl Fn()) {
        COMPOSER.with(|local_composer| {
            local_composer.borrow().compose_impl.as_ref().unwrap().borrow_mut().start_root();
            content();
            local_composer.borrow().compose_impl.as_ref().unwrap().borrow_mut().end_root();
        });
    }

    pub fn do_compose_validate_structure(content: impl Fn()) {
        COMPOSER.with(|local_composer| {
            local_composer.borrow().compose_impl.as_ref().unwrap().borrow_mut().start_root();
            content();
            local_composer.borrow().compose_impl.as_ref().unwrap().borrow_mut().end_root();
        });
    }

    pub fn start_restart_group() {
        Self::static_dispatch_mut(move |composer| composer.start_restart_group())
    }

    pub fn end_restart_group() -> Option<Rc<RefCell<dyn ScopeUpdateScope>>> {
        Self::static_dispatch_mut(move |composer| composer.end_restart_group())
    }

    pub fn skipping() -> bool {
        Self::static_dispatch(move |composer| composer.skipping())
    }

    pub fn skip_to_end() {
        Self::static_dispatch_mut(move |composer| composer.skip_to_end())
    }

    pub fn record_measure_or_layout_defer_action(action: impl FnOnce() + 'static) {
        Self::static_dispatch_mut(move |composer| composer.record_measure_or_layout_defer_action(action))
    }

    pub fn apply_measure_or_layout_defer_action() {
        Self::static_dispatch_mut(move |composer| composer.apply_measure_or_layout_defer_action())
    }
}