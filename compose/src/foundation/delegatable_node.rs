use crate::foundation::oop::any_converter::AnyConverter;
use std::fmt::Debug;

pub trait DelegatableNode: AnyConverter + Debug {
    // fn get_node(&self) -> Weak<RefCell<dyn Node>>;
}
