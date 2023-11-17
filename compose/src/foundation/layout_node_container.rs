use crate::foundation::modifier::Modifier;

#[derive(Debug)]
pub(crate) struct LayoutNodeContainer {
    pub(crate) current: Vec<Modifier>,
}

impl LayoutNodeContainer {
    pub(crate) fn new() -> Self {
        LayoutNodeContainer {
            current: vec![],
        }
    }
}
