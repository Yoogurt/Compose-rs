use std::any::Any;

use crate::foundation::delegatable_node::DelegatableNode;

pub trait ParentDataModifierNode : DelegatableNode {
    fn modify_parent_data(&mut self, parent_data: Option<Box<dyn Any>>) -> Option<Box<dyn Any>>;
}