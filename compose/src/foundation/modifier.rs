use crate::foundation::layout_modifier_node::LayoutModifierNode;
use crate::foundation::node_coordinator::NodeCoordinator;
use crate::foundation::oop::any_converter::AnyConverter;
use crate::foundation::utils::weak_upgrade::WeakUpdater;
use auto_delegate::delegate;
use compose_macro::Leak;
use std::any::Any;
use std::cell::RefCell;
use std::fmt::Debug;
use std::fmt::{Formatter, Write};
use std::ops::{Add, Deref};
use std::rc::{Rc, Weak};

pub const Modifier: Modifier = Modifier::Unit;

#[derive(Debug)]
pub enum NodeKind<'a> {
    Any(&'a mut dyn Node),
    LayoutModifierNode(&'a mut dyn LayoutModifierNode),
}

#[macro_export]
macro_rules! impl_node_kind_any {
    ($tt:tt) => {
        impl NodeKindPatch for $tt {
            fn get_node_kind(&mut self) -> NodeKind {
                NodeKind::Any(self)
            }
        }

        impl crate::foundation::oop::any_converter::AnyConverter for $tt {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };
}

pub trait NodeKindPatch {
    fn get_node_kind(&mut self) -> NodeKind;
}

#[delegate]
pub trait Node: NodeKindPatch + AnyConverter + Debug {
    fn set_parent(&mut self, parent: Option<Weak<RefCell<dyn Node>>>);
    fn get_parent(&self) -> Option<Rc<RefCell<dyn Node>>>;

    fn set_child(&mut self, parent: Option<Rc<RefCell<dyn Node>>>);
    fn get_child(&self) -> Option<Rc<RefCell<dyn Node>>>;

    fn update_coordinator(&mut self, coordinator: Option<Weak<RefCell<dyn NodeCoordinator>>>);
    fn get_coordinator(&self) -> Option<Weak<RefCell<dyn NodeCoordinator>>>;
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

impl AnyConverter for NodeImpl {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
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

    fn update_coordinator(&mut self, coordinator: Option<Weak<RefCell<dyn NodeCoordinator>>>) {
        self.coordinator = coordinator;
    }

    fn get_coordinator(&self) -> Option<Weak<RefCell<dyn NodeCoordinator>>> {
        self.coordinator.clone()
    }
}

#[derive(Default)]
pub enum Modifier {
    #[default]
    Unit,
    ModifierNodeElement {
        create: Box<dyn FnMut() -> Box<dyn LayoutModifierNode>>,
        update: Box<dyn FnMut(&'static mut Box<dyn LayoutModifierNode>)>,
    },
    Combined {
        left: Box<Modifier>,
        right: Box<Modifier>,
    },
}

impl Modifier {
    pub fn then(self, modifier: Modifier) -> Modifier {
        if let Modifier::Unit = self {
            return modifier;
        }

        if let Modifier::Unit = modifier {
            return self;
        }

        Modifier::Combined {
            left: Box::new(self),
            right: Box::new(modifier),
        }
    }

    pub fn fold_in<R>(&self, initial: R, mut operation: impl FnMut(R, &Modifier) -> R) -> R {
        match self {
            Modifier::Combined { left, right } => {
                right.fold_in(left.fold_in(initial, &mut operation), operation)
            }
            _ => operation(initial, self),
        }
    }

    pub fn fold_out<R>(&self, initial: R, operation: &mut dyn FnMut(&Modifier, R) -> R) -> R {
        match self {
            Modifier::Combined { left, right } => {
                left.fold_out(right.fold_out(initial, operation), operation)
            }
            _ => operation(self, initial),
        }
    }

    pub fn any(&self, mut predicate: impl FnMut(&Modifier) -> bool) -> bool {
        match self {
            Modifier::Combined { left, right } => left.any(&mut predicate) || right.any(predicate),
            _ => predicate(self),
        }
    }

    pub fn all(&self, mut predicate: impl FnMut(&Modifier) -> bool) -> bool {
        match self {
            Modifier::Combined { left, right } => left.all(&mut predicate) && right.all(predicate),
            _ => predicate(self),
        }
    }

    pub(crate) fn flatten(self) -> Vec<Modifier> {
        let mut result = Vec::<Modifier>::with_capacity(16);
        let mut stack: Vec<Modifier> = vec![self];

        while let Some(next) = stack.pop() {
            match next {
                Modifier::Combined { left, right } => {
                    stack.push(*right);
                    stack.push(*left);
                }

                _ => {
                    result.push(next);
                }
            }
        }

        result
    }
}

impl Modifier {
    const fn is_element(&self) -> bool {
        match self {
            Modifier::ModifierNodeElement { .. } => true,
            _ => false,
        }
    }
}

impl Add for Modifier {
    type Output = Modifier;
    fn add(self, rhs: Self) -> Self::Output {
        self.then(rhs)
    }
}

impl Debug for Modifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Modifier::Unit => f.write_str("<modifier:unit>"),
            Modifier::Combined { left, right } => {
                f.write_str("<modifier:combined>")?;
                left.fmt(f)?;
                right.fmt(f)
            }
            Modifier::ModifierNodeElement { create, update } => {
                f.write_str("<modifier:element[")?;
                f.write_str(&format!("create:{:p}", create.deref()))?;
                f.write_str(&format!(",update:{:p}]>", update.deref()))
            }
            _ => f.write_str("<unknown modifier>"),
        }
    }
}
