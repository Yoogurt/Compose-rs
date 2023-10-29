use crate::foundation::modifier::Modifier;
use crate::foundation::modifier_container::ModifierContainer;

impl ModifierContainer {
    pub(crate) fn new() -> Self {
        ModifierContainer {
            modifier: Modifier,
            current: vec![]
        }
    }

    pub(crate) fn set_modifier(&mut self, modifier: Modifier) {
        self.modifier = modifier
    }
}