use std::cell::RefCell;
use std::rc::Weak;

pub(crate) trait SelfReference {
    fn get_self(&self) -> Weak<RefCell<Self>>;
}