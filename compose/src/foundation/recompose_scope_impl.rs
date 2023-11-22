use crate::foundation::composer::ScopeUpdateScope;

pub trait RecomposeScope {
    fn invalidate(&self);
}

pub(crate) struct RecomposeScopeImpl {
    block: Option<Box<dyn FnMut()>>,
}

impl RecomposeScope for RecomposeScopeImpl {
    fn invalidate(&self) {
        todo!()
    }
}

impl RecomposeScopeImpl {
    pub(crate) fn new() -> Self {
        Self {
            block: None,
        }
    }

    pub(crate) fn start(&mut self, token: i32) {

    }
}

impl ScopeUpdateScope for RecomposeScopeImpl {
    fn update_scope(&mut self, block: Box<dyn FnMut()>) {
        self.block = Some(block);
    }
}