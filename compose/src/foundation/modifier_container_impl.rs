use crate::foundation::modifier::Modifier;
use crate::foundation::modifier_container::ModifierContainer;

impl ModifierContainer {
    pub(crate) fn new() -> Self {
        ModifierContainer {
            current: vec![]
        }
    }
}