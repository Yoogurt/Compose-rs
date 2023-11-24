use std::any::Any;

pub(crate) trait RememberObserver: Any {
    fn on_remembered(&self);
    fn on_forgotten(&self);
    fn on_abandoned(&self);
}

pub(crate) struct RememberObserverDelegate {}

impl RememberObserver for RememberObserverDelegate {
    fn on_remembered(&self) {}

    fn on_forgotten(&self) {}

    fn on_abandoned(&self) {}
}