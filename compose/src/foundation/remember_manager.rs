use std::cell::RefCell;
use std::rc::Rc;
use std::any::Any;
use crate::foundation::compose_node_lifecycle_callback::ComposeNodeLifecycleCallback;
use crate::foundation::remember_observer::{RememberObserver, RememberObserverDelegate};

pub(crate) trait RememberManager {
    fn remembering(&mut self, instance: Rc<RefCell<dyn RememberObserver>>);
    fn forgetting(&mut self, instance: Rc<RefCell<dyn RememberObserver>>);
    fn side_effects(&mut self, side_effect: Box<dyn FnOnce()>);

    fn reuse(&mut self, instance: Rc<RefCell<dyn ComposeNodeLifecycleCallback>>);
    fn deactivate(&mut self, instance: Rc<RefCell<dyn ComposeNodeLifecycleCallback>>);
    fn release(&mut self, instance: Rc<RefCell<dyn ComposeNodeLifecycleCallback>>);
}

#[derive(Default)]
pub(crate) struct RememberEventDispatcher {
    remembering: Vec<Rc<RefCell<dyn RememberObserver>>>,
    forgetting: Vec<Rc<RefCell<dyn RememberObserver>>>,
    abandon: Vec<Rc<RefCell<dyn RememberObserver>>>,
    side_effects: Vec<Box<dyn FnOnce()>>,
    reuse: Vec<Rc<RefCell<dyn ComposeNodeLifecycleCallback>>>,
    deactivate: Vec<Rc<RefCell<dyn ComposeNodeLifecycleCallback>>>,
    release: Vec<Rc<RefCell<dyn ComposeNodeLifecycleCallback>>>,
}

impl RememberEventDispatcher {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn dispatch_remember_observers(&mut self) {
        while let Some(deactivating) = self.deactivate.pop() {
            deactivating.borrow_mut().on_deactivate();
        }

        while let Some(forgetting) = self.forgetting.pop() {
            forgetting.borrow_mut().on_forgotten();
        }

        while let Some(remembering) = self.remembering.pop() {
            remembering.borrow_mut().on_remembered();
        }

        while let Some(release) = self.release.pop() {
            release.borrow_mut().on_release();
        }
    }

    pub(crate) fn dispatch_side_effects(&mut self) {
        while let Some(side_effect) = self.side_effects.pop() {
            side_effect();
        }
    }

    pub(crate) fn dispatch_abandoned(&mut self) {
        while let Some(abandon) = self.abandon.pop() {
            abandon.borrow_mut().on_abandoned();
        }
    }
}

impl RememberManager for RememberEventDispatcher {
    fn remembering(&mut self, instance: Rc<RefCell<dyn RememberObserver>>) {
        self.remembering.push(instance);
    }

    fn forgetting(&mut self, instance: Rc<RefCell<dyn RememberObserver>>) {
        self.forgetting.push(instance);
    }

    fn side_effects(&mut self, side_effect: Box<dyn FnOnce()>) {
        self.side_effects.push(side_effect);
    }

    fn reuse(&mut self, instance: Rc<RefCell<dyn ComposeNodeLifecycleCallback>>) {
        self.reuse.push(instance);
    }

    fn deactivate(&mut self, instance: Rc<RefCell<dyn ComposeNodeLifecycleCallback>>) {
        self.deactivate.push(instance);
    }

    fn release(&mut self, instance: Rc<RefCell<dyn ComposeNodeLifecycleCallback>>) {
        self.release.push(instance);
    }
}