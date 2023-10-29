use std::rc::{Rc, Weak};
use std::cell::{Ref, RefCell};
use std::mem::MaybeUninit;
use auto_delegate::Delegate;
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::modifier::{Node, NodeImpl};
use crate::foundation::modifier_container::ModifierContainer;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;

use super::inner_node_coordinator::InnerNodeCoordinator;

use super::modifier::Modifier;
use super::node_chain::NodeChain;

#[derive(Debug, Delegate, Default)]
struct TailModifierNode {
    #[to(Node)]
    node_impl: NodeImpl,
}

#[derive(Debug, Delegate, Default)]
struct SentineHeadNode {
    #[to(Node)]
    node_impl: NodeImpl,
}

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

            layout_node: MaybeUninit::uninit(),
        };

        result.wrap_with_rc_refcell()
    }

    pub(crate) fn attach(&mut self, layout_node: Weak<RefCell<LayoutNode>>, modifier_container: Rc<RefCell<ModifierContainer>>) {
        self.layout_node = MaybeUninit::new(layout_node.clone());
        self.modifier_container = modifier_container;
        self.inner_coordinator.borrow_mut().attach(layout_node);
    }

    fn pad_chain(&mut self) -> Rc<RefCell<dyn Node>> {
        let current_head = self.head.clone();
        current_head.borrow_mut().set_parent(Some(self.sentine_head.clone()));
        self.sentine_head.borrow_mut().set_child(Some(current_head.clone()));
        return self.sentine_head.clone();
    }

    fn insert_child(node: Rc<RefCell<dyn Node>>, parent:  Rc<RefCell<dyn Node>>) -> Rc<RefCell<dyn Node>>{
        {
            let mut parent_mut = parent.borrow_mut();
            let the_child = parent_mut.get_child();
            if let Some(the_child) = the_child {
                the_child.borrow_mut().set_parent(Some(node.clone()));
                node.borrow_mut().set_child(Some(the_child));
            }
            parent_mut.set_child(Some(node.clone()));
        }
        node.borrow_mut().set_parent(Some(parent));
        node
    }

    fn create_and_insert_node_as_child(element: &mut Modifier, parent: Rc<RefCell<dyn Node>>) -> Rc<RefCell<dyn Node>> {
        let node =match element {
            Modifier::ModifierNodeElement { create, update } => {
                 create()
            }

            _ => {
                todo!()
            }
        };

        Self::insert_child(node, parent)
    }

    pub(crate) fn update_from(&mut self, modifier: &mut Modifier) {
        // perform expensive reinit for modifier
        // todo structure update modifier
        let mut coordinator_sync_needed = false;

        let padded_head = self.pad_chain();

        let modifier_container = self.modifier_container.borrow_mut();
        let mut before = &modifier_container.current;
        let before_size = before.len();

        let mut after = modifier.flatten_mut();
        let after_size = after.len();

        let mut index = 0usize;

        if before_size == after_size {
            todo!()
        } else if before_size == 0 {
            coordinator_sync_needed = true;

            let mut node = padded_head;
            while index < after_size {
                let parent = node.clone();
                node = Self::create_and_insert_node_as_child(after[index], parent);
                index += 1;
            }
        } else if after_size == 0 {
            todo!()
        } else {
            todo!()
        }
    }
}