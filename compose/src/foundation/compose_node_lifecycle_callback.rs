pub(crate) trait ComposeNodeLifecycleCallback {
    fn on_reuse(&mut self);
    fn on_deactivate(&mut self);
    fn on_release(&mut self);
}