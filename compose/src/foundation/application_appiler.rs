use crate::foundation::applier::Applier;

pub(crate) struct ApplicationApplier {

}

impl ApplicationApplier {
    pub(crate) fn new() -> Self {
        Self {
        }
    }
}

impl Applier<()> for ApplicationApplier {
    fn get_current(&self) -> &() {
        &()
    }

    fn down(&mut self, node: ()) {
    }

    fn up(&mut self) {
    }

    fn clear(&mut self) {
    }
}