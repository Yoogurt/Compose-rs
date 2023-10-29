use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use auto_delegate::delegate;

pub const Modifier: Modifier = Modifier::Unit;

#[delegate]
pub trait Node: Debug + Any {
    fn set_parent(&mut self, parent: Option<Rc<RefCell<dyn Node>>>);
    fn get_parent(&self) -> Option<Rc<RefCell<dyn Node>>>;

    fn set_child(&mut self, parent: Option<Rc<RefCell<dyn Node>>>);
    fn get_child(&self) -> Option<Rc<RefCell<dyn Node>>>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug, Default)]
pub(crate) struct NodeImpl {
    parent: Option<Rc<RefCell<dyn Node>>>,
    child: Option<Rc<RefCell<dyn Node>>>,
}

impl Node for NodeImpl {
    fn set_parent(&mut self, parent: Option<Rc<RefCell<dyn Node>>>) {
        self.parent = parent;
    }

    fn get_parent(&self) -> Option<Rc<RefCell<dyn Node>>> {
        self.parent.clone()
    }

    fn set_child(&mut self, child: Option<Rc<RefCell<dyn Node>>>) {
        self.child = child;
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