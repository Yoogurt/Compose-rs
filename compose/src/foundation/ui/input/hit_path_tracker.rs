use crate::foundation::geometry::IntOffset;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use skia_safe::image_filters::merge;
use crate::foundation::layout::layout_coordinates::LayoutCoordinates;
use crate::foundation::modifier::{ModifierNode, ModifierNodeExtension, NodeKind};
use crate::foundation::modifier_node::PointerInputModifierNodeExtension;
use crate::foundation::ui::input::internal_pointer_event::InternalPointerEvent;
use crate::foundation::ui::input::pointer_event::{PointerId, PointerInputChange};

pub(crate) struct HitPathTracker {
    root: NodeParent,
}

struct Node {
    modifier_node: Rc<RefCell<dyn ModifierNode>>,
    coordinates: Option<Rc<RefCell<dyn LayoutCoordinates>>>,
    pointer_ids: Vec<PointerId>,

    is_in: bool,
    node_parent: NodeParent,
}

struct NodeParent {
    children: Vec<Node>,
}

impl Deref for Node {
    type Target = NodeParent;

    fn deref(&self) -> &Self::Target {
        &self.node_parent
    }
}

impl DerefMut for Node {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node_parent
    }
}

impl NodeParent {
    fn new() -> NodeParent {
        NodeParent {
            children: vec![]
        }
    }
}

impl Node {
    fn new(modifier_node: Rc<RefCell<dyn ModifierNode>>) -> Node {
        Node {
            modifier_node,
            coordinates: None,
            pointer_ids: vec![],

            is_in: false,
            node_parent: NodeParent::new(),
        }
    }

    fn build_cache(&mut self,
                   changes: &HashMap<PointerId, PointerInputChange>,
                   parent_coordinates: &Rc<RefCell<dyn LayoutCoordinates>>,
                   internal_pointer_event: &InternalPointerEvent,
                   is_in_bounds: bool) {
        let modifier_node = self.modifier_node.borrow();
        if !modifier_node.is_attach() {
            return;
        }

        modifier_node.dispatch_for_kind(NodeKind::PointerInput, |modifier_node| {
            self.coordinates = Some(PointerInputModifierNodeExtension::require_coordinator(modifier_node.as_pointer_input_modifier_node().unwrap(), NodeKind::PointerInput));
        });
    }

    pub(crate) fn mark_is_in(&mut self) {
        self.is_in = true;
    }

    pub(crate) fn dispatch_main_event_pass(&mut self, changes: HashMap<PointerId, PointerInputChange>, parent_coordinates: Rc<RefCell<dyn LayoutCoordinates>>, internal_pointer_event: InternalPointerEvent, is_in_bounds: bool) {}
}

impl HitPathTracker {
    pub(crate) fn new(root_coordinates: Rc<RefCell<dyn LayoutCoordinates>>) -> HitPathTracker {
        HitPathTracker {
            root: NodeParent::new()
        }
    }

    pub(crate) fn add_hit_path(&mut self, pointer_id: PointerId, mut pointer_input_node: Vec<Rc<RefCell<dyn ModifierNode>>>) {
        let mut parent = &mut self.root;

        let mut merging = false;

        for pointer_input_node in &pointer_input_node {
            if merging {
                let node = parent.children.iter_mut().find(|node| Rc::ptr_eq(&node.modifier_node, pointer_input_node));

                match node {
                    Some(node) => {
                        node.mark_is_in();

                        if node.pointer_ids.contains(&pointer_id) { node.pointer_ids.push(pointer_id); }
                        parent = node.deref_mut();
                        continue;
                    }
                    _ => {
                        merging = false;
                    }
                }
            }

            let mut node = Node::new(pointer_input_node.clone());
            node.pointer_ids.push(pointer_id);

            parent.children.push(node);
            parent = parent.children.last_mut().unwrap();
        }
    }

    // pub(crate) fn get_hit_path(&self, point: IntOffset) -> Vec<Rc<RefCell<dyn LayoutCoordinates>>> {
    //     let mut result = vec![];
    //     let mut current = self.root_coordinates.clone();
    //     while current.borrow().is_attached() {
    //         result.push(current.clone());
    //         current = current.borrow().get_parent_layout_coordinates().unwrap();
    //     }
    //     result
    // }
}