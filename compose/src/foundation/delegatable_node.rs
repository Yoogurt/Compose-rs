use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::rc::Weak;

use auto_delegate::delegate;
use crate::foundation::layout_node::LayoutNode;

use crate::foundation::modifier::ModifierNode;
use crate::foundation::oop::AnyConverter;

pub(crate) enum DelegatableKind {
    This,
    Other(Weak<RefCell<dyn ModifierNode>>),
}

#[delegate]
pub(crate) trait DelegatableNode: AnyConverter + Debug {
    fn get_node(&self) -> DelegatableKind;
}

pub(crate) trait ToDelegatedNode {
    fn to_delegated_node(&self) -> Rc<RefCell<dyn ModifierNode>>;
}

impl ToDelegatedNode for Rc<RefCell<dyn ModifierNode>> {
    fn to_delegated_node(&self) -> Rc<RefCell<dyn ModifierNode>> {
        match self.borrow().get_node() {
            DelegatableKind::This => {
                self.clone()
            }
            DelegatableKind::Other(other) => {
                other.upgrade().unwrap()
            }
        }
    }
}