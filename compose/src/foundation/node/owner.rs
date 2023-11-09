use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use crate::foundation::geometry::Density;
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::layout_direction::LayoutDirection;

pub(crate) trait Owner {
    fn get_root(&self) -> Rc<RefCell<LayoutNode>>;

    fn get_density(&self) -> Density;
    fn get_layout_direction(&self) -> LayoutDirection;

    fn on_request_relayout(&mut self, layout_node: Weak<RefCell<LayoutNode>>);
    fn on_attach(&self, layout_node: &LayoutNode);
    fn on_detach(&self, layout_node: &LayoutNode);
}