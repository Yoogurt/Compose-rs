use std::any::Any;
use crate::foundation::applier::Applier;
use crate::foundation::composer_impl::{Change, ChangeType};
use crate::foundation::remember_manager::{RememberEventDispatcher, RememberManager};
use crate::foundation::utils::box_wrapper::WrapWithBox;

pub(crate) struct Composition {
    applier: Box<dyn Applier<dyn Any>>,
    changes: Vec<Change>,
}

impl Composition {
    pub(crate) fn new(applier: impl Applier<dyn Any>) -> Self {
        Self {
            applier: applier.wrap_with_box(),
            changes: Vec::new(),
        }
    }

    pub(crate) fn record(&mut self, action: impl FnOnce(&mut dyn RememberManager) + 'static) {
        self.changes.push(Change {
            change: Box::new(action),
            change_type: ChangeType::Changes,
            // sequence: self.sequence,
        });
    }

    pub(crate) fn apply_changes(&mut self) {
        let mut changes = Vec::<Change>::new();
        std::mem::swap(&mut self.changes, &mut changes);

        let mut remember_dispatcher = RememberEventDispatcher::new();
        changes.into_iter().for_each(|change| {
            (change.change)(&mut remember_dispatcher);
        });
    }
}