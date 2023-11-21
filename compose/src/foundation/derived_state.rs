pub(crate) trait DerivedStateObserver {
    fn start(&mut self);
    fn done(&mut self);
}