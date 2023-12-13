use std::any::Any;

use crate::foundation::delegatable_node::DelegatableNode;
use crate::foundation::geometry::Density;

pub(crate) trait ParentDataModifierNode: DelegatableNode {
    fn modify_parent_data(&mut self, density: Density, parent_data: Option<Box<dyn Any>>) -> Option<Box<dyn Any>>;
}