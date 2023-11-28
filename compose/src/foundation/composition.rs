use crate::foundation::composer_impl::ApplierInType;
use std::any::Any;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use crate::foundation::applier::Applier;
use crate::foundation::composer_impl::{Change, ChangeType};
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::remember_manager::{RememberEventDispatcher, RememberManager};
use crate::foundation::utils::box_wrapper::WrapWithBox;

pub(crate) struct Composition {
    applier: Box<dyn Applier<Rc<RefCell<LayoutNode>>>>,
    changes: Vec<Change>,
    deferred_changes: Vec<Change>,
}

impl Composition {
    pub(crate) fn new(applier: impl Applier<Rc<RefCell<LayoutNode>>> + 'static) -> Self {
        Self {
            applier: applier.wrap_with_box(),
            changes: Vec::new(),
            deferred_changes: vec![],
        }
    }

    pub(crate) fn record(&mut self, action: impl FnOnce(&mut dyn Applier<ApplierInType>, &mut dyn RememberManager) + 'static) {
        self.changes.push(Change {
            change: Box::new(action),
            change_type: ChangeType::Changes,
        });
    }

    pub(crate) fn record_deferred_change(&mut self, deferred_change: impl FnOnce(&mut dyn Applier<ApplierInType>, &mut dyn RememberManager) + 'static) {
        self.deferred_changes.push(Change {
            change: Box::new(deferred_change),
            change_type: ChangeType::DeferredChange,
        });
    }

    pub(crate) fn apply_changes(&mut self) {
        let mut changes = Vec::<Change>::new();
        std::mem::swap(&mut self.changes, &mut changes);

        self.applier.on_begin_changes();

        let mut remember_dispatcher = RememberEventDispatcher::new();
        changes.into_iter().for_each(|change| {
            (change.change)(self.applier.deref_mut(), &mut remember_dispatcher);
        });

        self.applier.on_end_changes();
    }

    pub(crate) fn apply_deferred_changes(&mut self) {
        let mut deferred_changes = Vec::<Change>::new();
        std::mem::swap(&mut self.deferred_changes, &mut deferred_changes);

        let mut remember_dispatcher = RememberEventDispatcher::new();
        deferred_changes.into_iter().for_each(|change| {
            (change.change)(self.applier.deref_mut(), &mut remember_dispatcher);
        });
    }
}