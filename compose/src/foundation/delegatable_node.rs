use std::fmt::Debug;
use crate::foundation::r#trait::any_converter::AnyConverter;

pub trait DelegatableNode : AnyConverter + Debug {
    // fn get_node(&self) -> Weak<RefCell<dyn Node>>;
}