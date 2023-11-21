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