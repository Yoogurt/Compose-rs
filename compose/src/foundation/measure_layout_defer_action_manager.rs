use std::cell::RefCell;
use lazy_static::lazy_static;
use crate::foundation::utils::box_wrapper::WrapWithBox;

#[derive(Default)]
pub(crate) struct MeasureLayoutDeferActionManager {
    measure_defer_actions: Vec<Box<dyn FnOnce()>>,
    layout_defer_actions: Vec<Box<dyn FnOnce()>>,
}

thread_local! {
    static MANAGER: RefCell<Option<MeasureLayoutDeferActionManager>> = RefCell::new(None);
}

impl MeasureLayoutDeferActionManager {
    pub(crate) fn record_layout(action: impl FnOnce() + 'static) {
        MANAGER.with(move |manager| {
            manager.borrow_mut().as_mut().unwrap().layout_defer_actions.push(Box::new(action))
        });
    }

    pub(crate) fn record_measure(action: impl FnOnce() + 'static) {
        MANAGER.with(move |manager| {
            manager.borrow_mut().as_mut().unwrap().measure_defer_actions.push(Box::new(action))
        });
    }

    pub(crate) fn with_manager(defer_caller: impl FnOnce(Box<dyn FnOnce()>, Box<dyn FnOnce()>)) {
        MANAGER.with(|manager| {
            *manager.borrow_mut() = Some(MeasureLayoutDeferActionManager::default());
        });

        defer_caller((|| {
            MANAGER.with(|manager| {
                manager.borrow_mut().as_mut().unwrap().apply_measure_defer()
            })
        }).wrap_with_box(), (|| {
            MANAGER.with(|manager| {
                manager.borrow_mut().as_mut().unwrap().apply_layout_defer()
            })
        }).wrap_with_box());

        MANAGER.with(|manager| {
            *manager.borrow_mut() = None;
        });
    }

    fn apply_layout_defer(&mut self) {
        let mut layout_defer_actions = vec![];
        std::mem::swap(&mut layout_defer_actions, &mut self.layout_defer_actions);

        layout_defer_actions.into_iter().for_each(|action| {
            action();
        });
    }

    fn apply_measure_defer(&mut self) {
        let mut measure_defer_actions = vec![];
        std::mem::swap(&mut measure_defer_actions, &mut self.measure_defer_actions);

        measure_defer_actions.into_iter().for_each(|action| {
            action();
        });
    }
}