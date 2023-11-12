use std::cell::RefCell;
use std::rc::Rc;
use crate::foundation::oop::AnyConverter;
use std::fmt::Debug;
use std::rc::Weak;
use auto_delegate::delegate;
use crate::foundation::modifier::ModifierNode;

pub enum DelegatableKind {
    This,
    Other(Weak<RefCell<dyn ModifierNode>>),
}

#[delegate]
pub trait DelegatableNode: AnyConverter + Debug {
    fn get_node(&self) -> DelegatableKind;
}

pub(crate) trait ToDelegatedNode {
    fn to_delegated_node(&self) -> Rc<RefCell<dyn ModifierNode>>;
}

impl ToDelegatedNode for Rc<RefCell<dyn ModifierNode>> {
    fn to_delegated_node(&self) -> Rc<RefCell<dyn ModifierNode>> {
        match  self.borrow().get_node() {
            DelegatableKind::This => {
                self.clone()
            }
            DelegatableKind::Other(other) => {
                other.upgrade().unwrap()
            }
        }
    }
}