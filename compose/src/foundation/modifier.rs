use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::{Rc, Weak};
use compose_macro::Leak;
use auto_delegate::delegate;
use crate::foundation::delegatable_node::DelegatableNode;
use crate::foundation::layout_modifier_node::LayoutModifierNode;
use crate::foundation::look_ahead_capable_placeable::NodeCoordinator;
use crate::foundation::memory::leak_token::{LeakToken, LeakableObject};
use crate::foundation::utils::weak_upgrade::WeakUpdater;

pub const Modifier: Modifier = Modifier::Unit;

#[derive(Debug)]
pub enum NodeKind<'a> {
    Any(&'a mut dyn Node),
    // DelegatingNode(&'a mut dyn DelegatableNode),
    LayoutMidifierNode(&'a mut dyn LayoutModifierNode),
}

#[macro_export]
macro_rules! impl_node_kind_any {
    ($tt:tt) => {
        impl NodeKindPatch for $tt {
            fn get_node_kind(&mut self) -> NodeKind {
                NodeKind::Any(self)
            }
        }
    };
}

pub trait NodeKindPatch {
    fn get_node_kind(&mut self) -> NodeKind;
}

#[delegate]
pub trait Node: NodeKindPatch + Debug + Any {
    fn set_parent(&mut self, parent: Option<Weak<RefCell<dyn Node>>>);
    fn get_parent(&self) -> Option<Rc<RefCell<dyn Node>>>;

    fn set_child(&mut self, parent: Option<Rc<RefCell<dyn Node>>>);
    fn get_child(&self) -> Option<Rc<RefCell<dyn Node>>>;

    fn update_coordinator(&mut self, coordinator: Option<Weak<RefCell<dyn NodeCoordinator>>>);
    fn get_coordinator(&self) -> Option<Weak<RefCell<dyn NodeCoordinator>>>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[Leak]
#[derive(Debug, Default)]
pub(crate) struct NodeImpl {
    parent: Option<Weak<RefCell<dyn Node>>>,
    child: Option<Rc<RefCell<dyn Node>>>,
    coordinator: Option<Weak<RefCell<dyn NodeCoordinator>>>,
}

impl NodeKindPatch for NodeImpl {
    fn get_node_kind(&mut self) -> NodeKind {
        todo!("implement get node kind by yourself")
    }
}

impl Node for NodeImpl {
    fn set_parent(&mut self, parent: Option<Weak<RefCell<dyn Node>>>) {
        self.parent = parent;
    }

    fn get_parent(&self) -> Option<Rc<RefCell<dyn Node>>> {
        self.parent.try_upgrade()
    }

    fn set_child(&mut self, child: Option<Rc<RefCell<dyn Node>>>) {
        self.child = child
    }

    fn get_child(&self) -> Option<Rc<RefCell<dyn Node>>> {
       self.child.clone()
    }

    fn as_any(&self) -> &dyn Any where Self: Sized {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any where Self: Sized {
        self
    }

    fn get_coordinator(&self) -> Option<Weak<RefCell<dyn NodeCoordinator>>> {
        self.coordinator.clone()
    }

    fn update_coordinator(&mut self, coordinator: Option<Weak<RefCell<dyn NodeCoordinator>>>) {
        self.coordinator = coordinator;
    }
}


#[derive(Default)]
pub enum Modifier {
    #[default]
    Unit,
    ModifierNodeElement {
        create: Box<dyn FnMut() -> Rc<RefCell<dyn Node>>>,
        update: Box<dyn FnMut(&'static Rc<RefCell<dyn Node>>)>,
    },
    Combined {
        left: Box<Modifier>,
        right: Box<Modifier>,
    },
}