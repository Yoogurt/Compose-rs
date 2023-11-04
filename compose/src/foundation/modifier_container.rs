use crate::foundation::modifier::Modifier;

#[derive(Debug)]
pub(crate) struct ModifierContainer {
    pub(crate) current: Vec<Modifier>,
}

impl ModifierContainer {
    pub(crate) fn new() -> Self {
        ModifierContainer { current: vec![] }
    }
}
