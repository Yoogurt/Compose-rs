use std::any::Any;
use crate::foundation::applier::Applier;

pub(crate) struct ApplicationApplier {
    stub: Box<dyn Any>
}

impl ApplicationApplier {
    pub(crate) fn new() -> Self {
        Self {
            stub: Box::new(())
        }
    }
}

impl Applier<Box<dyn Any>> for ApplicationApplier {
    fn get_current(&self) -> &Box<dyn Any> {
        &self.stub
    }

    fn down(&mut self, node: Box<dyn Any>) {
    }

    fn up(&mut self) {
    }

    fn clear(&mut self) {
    }
}