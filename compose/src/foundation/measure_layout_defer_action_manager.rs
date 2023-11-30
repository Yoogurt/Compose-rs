use std::cell::RefCell;
use lazy_static::lazy_static;

#[derive(Default)]
pub(crate) struct MeasureLayoutDeferActionManager {
    measure_defer_actions: Vec<Box<dyn FnOnce()>>,
    layout_defer_actions: Vec<Box<dyn FnOnce()>>,
}

thread_local! {
    static MANAGER: RefCell<Option<MeasureLayoutDeferActionManager>> = RefCell::new(None);
}

pub(crate) type DeferCaller = (Box<dyn FnOnce()>, )

impl MeasureLayoutDeferActionManager {
    pub(crate) fn record_layout(&mut self, action: impl FnOnce() + 'static) {
        self.layout_defer_actions.push(Box::new(action))
    }

    pub(crate) fn record_measure(&mut self, action: impl FnOnce() + 'static) {
        self.measure_defer_actions.push(Box::new(action))
    }

    pub(crate) fn with_manager() {
        MANAGER.with(|mut manager| {
            *manager.borrow_mut() = Some(MeasureLayoutDeferActionManager::default());
            *manager.borrow_mut() = None;
        });
    }

    pub(crate) fn apply_layout_defer(&mut self) {
        let mut layout_defer_actions = vec![];
        std::mem::swap(&mut layout_defer_actions, &mut self.layout_defer_actions);

        layout_defer_actions.into_iter().for_each(|action| {
            action();
        });
    }

    pub(crate) fn apply_measure_defer(&mut self) {
        let mut measure_defer_actions = vec![];
        std::mem::swap(&mut measure_defer_actions, &mut self.measure_defer_actions);

        measure_defer_actions.into_iter().for_each(|action| {
            action();
        });
    }
}