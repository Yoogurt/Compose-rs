pub(crate) trait RememberObserver {
    fn on_remembered(&self);
    fn on_forgotten(&self);
    fn on_abandoned(&self);
}

pub(crate) struct RememberObserverItem {}