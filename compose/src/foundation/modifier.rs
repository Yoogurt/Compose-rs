use std::any::Any;
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

    fn as_any(&self) -> &dyn Any where Self: Sized {
        self
    }

    fn as_any_mut(&mut self) -> &dyn Any where Self: Sized {
        self
    }
}

// impl dyn Node {
//     pub fn downcast_ref<T>(&self) -> Option<& T> where Self: 'static, T: Any + 'static {
//         self as &dyn Any;
//
//         // let a: Option<&T> = (&self as &dyn Any).downcast_ref::<T>();
//         // a
//         todo!()
//     }
//
//     pub fn test(&self) {}
//
//     // pub fn downcast_mut<T>(&mut self) -> Option<&mut T> where T: Sized {
//     //     (&mut self as &mut dyn Any).downcast_mut::<T>()
//     // }
// }

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