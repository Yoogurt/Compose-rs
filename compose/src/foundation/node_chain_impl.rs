use std::rc::{Rc, Weak};
use std::cell::RefCell;
use auto_delegate::Delegate;
use crate::foundation::layout_modifier_node::LayoutModifierNode;
use crate::foundation::layout_modifier_node_coordinator::LayoutModifierNodeCoordinator;
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::look_ahead_capable_placeable::NodeCoordinator;
use crate::foundation::modifier::{Node, NodeImpl, NodeKind, NodeKindPatch};
use crate::foundation::modifier_container::ModifierContainer;
use crate::foundation::utils::optional_weak::OptionalWeak;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;
use crate::impl_node_kind_any;

use super::inner_node_coordinator::InnerNodeCoordinator;

use super::modifier::Modifier;
use super::node_chain::NodeChain;

#[derive(Debug, Delegate, Default)]
struct TailModifierNode {
    #[to(Node)]
    node_impl: NodeImpl,
}
impl_node_kind_any!(TailModifierNode);

#[derive(Debug, Default, Delegate)]
struct SentineHeadNode {
    #[to(Node)]
    node_impl: NodeImpl,
}
impl_node_kind_any!(SentineHeadNode);

impl NodeChain {
    pub(crate) fn new() -> Rc<RefCell<Self>> {
        let inner_node_coordinator = InnerNodeCoordinator::new().wrap_with_rc_refcell();

        let node = TailModifierNode::default().wrap_with_rc_refcell();

        let result = NodeChain {
            sentine_head: SentineHeadNode::default().wrap_with_rc_refcell(),

            head: node.clone(),
            tail: node,
            parent_data: None,
            modifier_container: ModifierContainer::new().wrap_with_rc_refcell(),
            measure_result: Default::default(),
            inner_coordinator: inner_node_coordinator.clone(),
            outer_coordinator: inner_node_coordinator,

            parent: Default::default(),

            layout_node: Weak::new(),
        };

        result.wrap_with_rc_refcell()
    }

    pub(crate) fn set_parent<T: Into<Weak<RefCell<LayoutNode>>>>(&mut self, parent: T) {
        self.parent = parent.into();
    }

    pub(crate) fn get_parent(&self) -> Weak<RefCell<LayoutNode>> {
        self.parent.clone()
    }

    pub(crate) fn attach(&mut self, layout_node: Weak<RefCell<LayoutNode>>, modifier_container: Rc<RefCell<ModifierContainer>>) {
        self.layout_node = layout_node.clone();
        self.modifier_container = modifier_container;
        self.inner_coordinator.borrow_mut().attach(layout_node);
    }

    fn pad_chain(&mut self) -> Rc<RefCell<dyn Node>> {
        let current_head = self.head.clone();
        current_head.borrow_mut().set_parent(Some(Rc::downgrade(&self.sentine_head)));
        self.sentine_head.borrow_mut().set_child(Some(current_head.clone()));
        return self.sentine_head.clone();
    }

    fn insert_child(node: Rc<RefCell<dyn Node>>, parent: Rc<RefCell<dyn Node>>) -> Rc<RefCell<dyn Node>> {
        {
            let mut parent_mut = parent.borrow_mut();
            let the_child = parent_mut.get_child();
            if let Some(the_child) = the_child {
                the_child.borrow_mut().set_parent(Some(Rc::downgrade(&node)));
                node.borrow_mut().set_child(Some(the_child.clone()));
            }
            parent_mut.set_child(Some(node.clone()));
        }
        node.borrow_mut().set_parent(Some(Rc::downgrade(&parent)));
        node
    }

    fn create_and_insert_node_as_child(element: &mut Modifier, parent: Rc<RefCell<dyn Node>>) -> Rc<RefCell<dyn Node>> {
        let node = match element {
            Modifier::ModifierNodeElement { create, update } => {
                create()
            }

            _ => {
                todo!()
            }
        };

        Self::insert_child(node, parent)
    }

    fn trim_chain(&mut self, padded_head: Rc<RefCell<dyn Node>>) -> Rc<RefCell<dyn Node>> {
        if padded_head.as_ptr() != self.sentine_head.as_ptr() {
            panic!("trim_chain called on already trimmed chain")
        }
        let result = self.sentine_head.borrow().get_child().unwrap_or(self.tail.clone());
        result.borrow_mut().set_parent(None);
        {
            let mut sentine_head_mut = self.sentine_head.borrow_mut();
            sentine_head_mut.set_child(None);
            sentine_head_mut.update_coordinator(None)
        }

        if result.as_ptr() == self.sentine_head.as_ptr() {
            panic!("trim_chain did not update the head")
        }

        result
    }

    fn node_as_layout_modifier_node<'a>(mut node_kind: NodeKind<'a>) -> Option<&'a mut dyn LayoutModifierNode> {
        match node_kind {
            NodeKind::LayoutMidifierNode(result) => {
                Some(result)
            }
            _ => {
                None
            }
        }
    }

    fn sync_coordinators(&mut self) {
        let mut coordinator: Rc<RefCell<dyn NodeCoordinator>> = self.inner_coordinator.clone();
        let mut node = self.tail.clone().borrow().get_parent();

        while let Some(node_rc) = node {
            let mut node_mut = node_rc.borrow_mut();
            let node_coordinator = node_mut.get_coordinator();

            let layout_node = Self::node_as_layout_modifier_node(node_mut.get_node_kind());

            if let Some(layout_mod) = layout_node {
                let next = if let Some(node_coordinator) = node_coordinator {
                    let node_coordinator = node_coordinator.upgrade().expect("upgrade fail from node coordinator");
                    let mut node_coordinator_mut = node_coordinator.borrow_mut();
                    let c = node_coordinator_mut.as_any_mut().downcast_mut::<LayoutModifierNodeCoordinator>().expect("coordinator with wrong type");
                    if node_rc.as_ptr() != c.set_layout_modifier_node(node_rc.clone()).as_ptr() {
                        c.on_layout_modifier_node_changed();
                    }
                    node_coordinator.clone()
                } else {
                    let c = LayoutModifierNodeCoordinator::new(self.layout_node.clone(), node_rc.clone()).wrap_with_rc_refcell();
                    let weak_layout_modifier_node_coordinator = Rc::downgrade(&c);
                    let weak_dyn_node_coordinator: Weak<RefCell<dyn NodeCoordinator>> = weak_layout_modifier_node_coordinator;
                    node_mut.update_coordinator(Some(weak_dyn_node_coordinator));
                    c
                };

                {
                    let mut coordinator_mut = coordinator.borrow_mut();
                    coordinator_mut.set_wrapped_by(Some(Rc::downgrade(&next)));
                    next.borrow_mut().set_wrapped(Some(coordinator.clone()));
                }
                coordinator = next;
            } else {
                let weak_dyn_node_coordinator = Rc::downgrade(&coordinator);
                node_mut.update_coordinator(Some(weak_dyn_node_coordinator))
            }

            node = node_mut.get_parent();
        }

        coordinator.borrow_mut().set_wrapped_by(self.parent.upgrade().and_then(|parent_layout_node| {
            let parent_inner_coordinator = Rc::downgrade(&parent_layout_node.borrow().node_chain.borrow().inner_coordinator);
            let parent_dyn_node_coordinator: Weak<RefCell<dyn NodeCoordinator>> = parent_inner_coordinator;
            Some(parent_dyn_node_coordinator)
        }));

        self.outer_coordinator = coordinator;
    }

    pub(crate) fn update_from(&mut self, mut modifier: Modifier) {
        // perform expensive reinit for modifier
        // todo structure update modifier
        let mut coordinator_sync_needed = false;
        let padded_head = self.pad_chain();

        {
            let mut modifier_container = self.modifier_container.borrow_mut();
            let mut before = &modifier_container.current;
            let before_size = before.len();

            let mut after = modifier.flatten();
            let after_size = after.len();

            let mut index = 0usize;

            if before_size == after_size {
                todo!()
            } else if before_size == 0 {
                coordinator_sync_needed = true;

                let mut node = padded_head.clone();
                while index < after_size {
                    let parent = node.clone();
                    node = Self::create_and_insert_node_as_child(&mut after[index], parent);
                    index += 1;
                }
            } else if after_size == 0 {
                todo!()
            } else {
                todo!()
            }

            dbg!(&after);
            modifier_container.current = after;
        }
        self.head = self.trim_chain(padded_head);

        if coordinator_sync_needed {
            self.sync_coordinators();
        }
    }
}