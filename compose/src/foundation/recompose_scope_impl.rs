pub trait RecomposeScope {
    fn invalidate(&self);
}

pub(crate) struct RecomposeScopeImpl {
}

impl RecomposeScope for RecomposeScopeImpl {
    fn invalidate(&self) {
        todo!()
    }
}

impl RecomposeScopeImpl {
    pub(crate) fn new() -> Self {
        Self {
        }
    }

    pub(crate) fn start(&mut self, token: i32) {

    }
}