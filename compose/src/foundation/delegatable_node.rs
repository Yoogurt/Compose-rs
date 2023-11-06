use crate::foundation::oop::AnyConverter;
use std::fmt::Debug;

pub trait DelegatableNode: AnyConverter + Debug {
    // fn get_node(&self) -> Weak<RefCell<dyn Node>>;
}
