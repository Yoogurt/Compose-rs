use std::{cell::RefCell, rc::Rc};
use std::cell::RefMut;
use std::ops::DerefMut;
use std::rc::Weak;

use auto_delegate::Delegate;
use compose_foundation_macro::ModifierElement;

use crate::foundation::layout_modifier_node_coordinator::LayoutModifierNodeCoordinator;
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::layout_node_container::LayoutNodeContainer;
use crate::foundation::measure_pass_delegate::MeasurePassDelegate;
use crate::foundation::modifier::{ModifierElement, ModifierNode};
use crate::foundation::modifier::{ModifierNodeImpl, NodeKind, NodeKindPatch};
use crate::foundation::modifier_node::LayoutModifierNode;
use crate::foundation::node::BackwardsCompatNode;
use crate::foundation::node_coordinator::TailModifierNodeProvider;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;
use crate::foundation::utils::self_reference::SelfReference;
use crate::impl_node_kind_any;

use super::{
    inner_node_coordinator::InnerNodeCoordinator, measure_result::MeasureResult,
    node_coordinator::NodeCoordinator, parent_data::ParentData,
};
use super::modifier::Modifier;

#[derive(Debug, Delegate, Default, ModifierElement)]
pub(crate) struct TailModifierNode {
    #[to(ModifierNode, DelegatableNode)]
    node_impl: ModifierNodeImpl,
}

#[derive(Debug)]
pub(crate) struct NodeChain {
    pub(crate) sentine_head: Rc<RefCell<dyn ModifierNode>>,

    pub(crate) head: Rc<RefCell<dyn ModifierNode>>,

    pub(crate) modifier_container: Rc<RefCell<LayoutNodeContainer>>,
    pub(crate) parent_data: Option<Box<dyn ParentData>>,
    pub(crate) measure_result: MeasureResult,
    pub(crate) inner_coordinator: Rc<RefCell<InnerNodeCoordinator>>,
    pub(crate) outer_coordinator: Rc<RefCell<dyn NodeCoordinator>>,

    pub(crate) parent: Option<Weak<RefCell<LayoutNode>>>,
    pub(crate) layout_node: Weak<RefCell<LayoutNode>>,

    weak_self: Weak<RefCell<NodeChain>>,
    identify: u32,
}
impl_node_kind_any!(TailModifierNode);

#[derive(Debug, Default, Delegate, ModifierElement)]
struct SentineHeadNode {
    #[to(ModifierNode, DelegatableNode)]
    node_impl: ModifierNodeImpl,
}
impl_node_kind_any!(SentineHeadNode);

impl SelfReference for NodeChain {
    fn get_self(&self) -> Weak<RefCell<Self>> {
        self.weak_self.clone()
    }
}

impl NodeChain {
    pub(crate) fn new() -> Rc<RefCell<Self>> {
        let inner_node_coordinator = InnerNodeCoordinator::new();
        let head = inner_node_coordinator.borrow().get_tail();

        let result = NodeChain {
            sentine_head: SentineHeadNode::default().wrap_with_rc_refcell(),

            head,
            parent_data: None,
            modifier_container: LayoutNodeContainer::new().wrap_with_rc_refcell(),
            measure_result: Default::default(),
            inner_coordinator: inner_node_coordinator.clone(),
            outer_coordinator: inner_node_coordinator,

            parent: Default::default(),

            layout_node: Weak::new(),
            weak_self: Weak::new(),

            identify: 0,
        }.wrap_with_rc_refcell();

        result.borrow_mut().weak_self = Rc::downgrade(&result);
        result
    }

    pub(crate) fn set_parent(&mut self, parent: Option<Weak<RefCell<LayoutNode>>>) {
        self.parent = parent;
    }

    pub(crate) fn get_parent(&self) -> Option<Weak<RefCell<LayoutNode>>> {
        self.parent.clone()
    }

    pub(crate) fn attach(
        &mut self,
        identify: u32,
        layout_node: &Rc<RefCell<LayoutNode>>,
        modifier_container: &Rc<RefCell<LayoutNodeContainer>>,
        measure_pass_delegate: &Rc<RefCell<MeasurePassDelegate>>,
        node_chain: &Rc<RefCell<NodeChain>>,
    ) {
        self.identify = identify;
        self.layout_node = Rc::downgrade(layout_node);
        self.modifier_container = modifier_container.clone();
        self.inner_coordinator.borrow_mut().attach(identify, layout_node, measure_pass_delegate, node_chain);
    }

    fn pad_chain(&mut self) -> Rc<RefCell<dyn ModifierNode>> {
        let current_head = self.head.clone();
        current_head
            .borrow_mut()
            .set_parent(Some(Rc::downgrade(&self.sentine_head)));
        self.sentine_head
            .borrow_mut()
            .set_child(Some(current_head.clone()));
        return self.sentine_head.clone();
    }

    fn insert_child(
        node: Rc<RefCell<dyn ModifierNode>>,
        parent: Rc<RefCell<dyn ModifierNode>>,
    ) -> Rc<RefCell<dyn ModifierNode>> {
        {
            let mut parent_mut = parent.borrow_mut();
            let the_child = parent_mut.get_child();
            if let Some(the_child) = the_child {
                the_child
                    .borrow_mut()
                    .set_parent(Some(Rc::downgrade(&node)));
                node.borrow_mut().set_child(Some(the_child.clone()));
            }
            parent_mut.set_child(Some(node.clone()));
        }
        node.borrow_mut().set_parent(Some(Rc::downgrade(&parent)));
        node
    }

    fn create_and_insert_node_as_child(
        element: &mut Modifier,
        parent: Rc<RefCell<dyn ModifierNode>>,
    ) -> Rc<RefCell<dyn ModifierNode>> {
        let node = match element {
            Modifier::ModifierNodeElement { create, update } => create(),
            Modifier::ModifierElement(element) => {
                BackwardsCompatNode::new(element.clone()).wrap_with_rc_refcell()
            }
            _ => {
                todo!()
            }
        };

        Self::insert_child(node, parent)
    }

    fn get_tail(&self) -> Rc<RefCell<dyn ModifierNode>> {
        self.inner_coordinator.borrow().get_tail()
    }

    pub(crate) fn tail_to_head(&mut self, mut block: impl FnMut(&mut dyn ModifierNode)) {
        let mut node = Some(self.get_tail().clone());
        while let Some(node_rc) = node {
            block(node_rc.borrow_mut().deref_mut());
            node = node_rc.borrow().get_parent();
        }
    }

    fn trim_chain(&mut self, padded_head: Rc<RefCell<dyn ModifierNode>>) -> Rc<RefCell<dyn ModifierNode>> {
        if padded_head.as_ptr() != self.sentine_head.as_ptr() {
            panic!("trim_chain called on already trimmed chain")
        }
        let result = self
            .sentine_head
            .borrow()
            .get_child()
            .unwrap_or(self.get_tail().clone());
        result.borrow_mut().set_parent(None);
        {
            let mut sentine_head_mut = self.sentine_head.borrow_mut();
            sentine_head_mut.set_child(None);
            sentine_head_mut.update_coordinator(None)
        }

        if result.as_ptr() as *const () == self.sentine_head.as_ptr() as *const () {
            panic!("trim_chain did not update the head")
        }

        result
    }

    fn node_as_layout_modifier_node<'a, 'b>(
        mut node: &'b mut RefMut<'a, dyn ModifierNode>,
    ) -> Option<&'b mut dyn LayoutModifierNode> where 'a: 'b {
        let node_kind = node.get_node_kind();
        match node_kind {
            NodeKind::Layout => node.as_layout_modifier_node_mut(),
            _ => {
                println!("unknown type: {:?}", node_kind);
                None
            }
        }
    }

    fn sync_coordinators(&mut self) {
        let mut coordinator: Rc<RefCell<dyn NodeCoordinator>> = self.inner_coordinator.clone();
        let mut node = self.get_tail().borrow().get_parent();

        while let Some(node_rc) = node {
            let mut node_mut = node_rc.borrow_mut();
            let node_coordinator = node_mut.get_coordinator();

            let layout_node = Self::node_as_layout_modifier_node(&mut node_mut);

            if let Some(layout_mod) = layout_node {
                let next = if let Some(node_coordinator) = node_coordinator {
                    let node_coordinator = node_coordinator
                        .upgrade()
                        .expect("upgrade fail from node coordinator");
                    let mut node_coordinator_mut = node_coordinator.borrow_mut();
                    let c = node_coordinator_mut
                        .as_any_mut()
                        .downcast_mut::<LayoutModifierNodeCoordinator>()
                        .expect("coordinator with wrong type");
                    if node_rc.as_ptr() != c.set_layout_modifier_node(node_rc.clone()).as_ptr() {
                        c.on_layout_modifier_node_changed();
                    }
                    node_coordinator.clone()
                } else {
                    let c = LayoutModifierNodeCoordinator::new(
                        &self.layout_node.upgrade().unwrap(),
                        &node_rc,
                        &self.get_self().upgrade().unwrap(),
                    );
                    let weak_layout_modifier_node_coordinator = Rc::downgrade(&c);
                    let weak_dyn_node_coordinator: Weak<RefCell<dyn NodeCoordinator>> =
                        weak_layout_modifier_node_coordinator;
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

        coordinator
            .borrow_mut()
            .set_wrapped_by(self.parent.as_ref().unwrap_or(&Weak::default()).upgrade().and_then(|parent_layout_node| {
                let parent_inner_coordinator = Rc::downgrade(
                    &parent_layout_node
                        .borrow()
                        .node_chain
                        .borrow()
                        .inner_coordinator,
                );
                let parent_dyn_node_coordinator: Weak<RefCell<dyn NodeCoordinator>> =
                    parent_inner_coordinator;
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
                    let parent = node;
                    node = Self::create_and_insert_node_as_child(&mut after[index], parent);
                    index += 1;
                }
            } else if after_size == 0 {
                todo!()
            } else {
                todo!()
            }

            modifier_container.current = after;
        }
        self.head = self.trim_chain(padded_head);

        if coordinator_sync_needed {
            self.sync_coordinators();
        }
    }

    pub(crate) fn for_each_coordinator(
        &self,
        mut block: impl FnMut(&LayoutModifierNodeCoordinator),
    ) {
        let mut coordinator = self.outer_coordinator.clone();
        let inner_coordinator = self.inner_coordinator.clone();

        let mut coordinator_ptr = coordinator.as_ptr() as *const ();
        let inner_coordinator_ptr = inner_coordinator.as_ptr() as *const ();

        while coordinator_ptr != inner_coordinator_ptr {
            block(
                coordinator
                    .borrow()
                    .as_any()
                    .downcast_ref::<LayoutModifierNodeCoordinator>()
                    .unwrap(),
            );
            let wrapped = coordinator.borrow().get_wrapped().unwrap();
            coordinator = wrapped;
            coordinator_ptr = coordinator.as_ptr() as *const ();
        }
    }
}
