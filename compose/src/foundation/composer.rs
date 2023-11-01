use std::{rc::Rc, cell::RefCell};
use crate::foundation::composer_inner::ComposerInner;

pub struct Composer {
    pub(crate) inner: RefCell<ComposerInner>,
}