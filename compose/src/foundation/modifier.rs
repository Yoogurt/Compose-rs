#![allow(non_upper_case_globals)]

use std::cell::{RefCell, RefMut};
use std::fmt::{Formatter, Write};
use std::fmt::Debug;
use std::ops::{Add, BitAnd, Deref};
use std::rc::{Rc, Weak};

use auto_delegate::delegate;
use compose_foundation_macro::{Leak, ModifierElement};

use crate::foundation::delegatable_node::{DelegatableKind, DelegatableNode};
use crate::foundation::node_coordinator::NodeCoordinator;
use crate::foundation::oop::{AnyConverter, DrawModifierNodeConverter, ParentDataModifierNodeConverter};
use crate::foundation::oop::LayoutModifierNodeConverter;
use crate::foundation::utils::box_wrapper::WrapWithBox;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;
use crate::foundation::utils::weak_upgrade::WeakUpdater;

pub const Modifier: Modifier = Modifier::Unit;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NodeKind {
    Any = 1 << 0,
    Layout = 1 << 1,
    ParentData = 1 << 2,
    Draw = 1 << 3,
    LayoutAware = 1 << 4,
}

impl NodeKind {
    pub fn mask(&self) -> u32 {
        *self as u32
    }

    pub(crate) fn include_self_in_traversal(&self) -> bool {
        NodeKind::LayoutAware & self.mask() != 0
    }
}

impl From<NodeKind> for u32 {
    fn from(value: NodeKind) -> Self {
        value as u32
    }
}

impl BitAnd<u32> for NodeKind {
    type Output = u32;
    fn bitand(self, rhs: u32) -> Self::Output {
        u32::from(self) & rhs
    }
}

#[macro_export]
macro_rules! impl_node_kind_any {
    ($tt:tt) => {
        impl NodeKindPatch for $tt {
            fn get_node_kind(&self) -> NodeKind {
                NodeKind::Any
            }
        }
    };
}

#[macro_export]
macro_rules! impl_node_kind_parent_data {
    ($tt:tt) => {
        impl NodeKindPatch for $tt {
            fn get_node_kind(&self) -> NodeKind {
                NodeKind::ParentData
            }
        }

        impl DelegatableNode for $tt {
            fn get_node(&self) -> DelegatableKind {
                DelegatableKind::This
            }
        }
    };
}

pub trait NodeKindPatch {
    fn get_node_kind(&self) -> NodeKind;
}

pub trait NodeKindParentData: NodeKindPatch {
    fn get_node_kind(&self) -> NodeKind {
        NodeKind::ParentData
    }
}

pub trait ModifierElement: AnyConverter + LayoutModifierNodeConverter + DrawModifierNodeConverter + ParentDataModifierNodeConverter + NodeKindPatch + Debug {
    fn as_modifier_element(&self) -> &dyn ModifierElement;
    fn as_modifier_element_mut(&mut self) -> &mut dyn ModifierElement;
}

#[delegate]
pub trait ModifierNode: ModifierElement + DelegatableNode {
    fn set_parent(&mut self, parent: Option<Weak<RefCell<dyn ModifierNode>>>);

    fn get_parent(&self) -> Option<Rc<RefCell<dyn ModifierNode>>>;

    fn set_child(&mut self, parent: Option<Rc<RefCell<dyn ModifierNode>>>);

    fn get_child(&self) -> Option<Rc<RefCell<dyn ModifierNode>>>;

    fn update_coordinator(&mut self, coordinator: Option<Weak<RefCell<dyn NodeCoordinator>>>);

    fn get_coordinator(&self) -> Option<Weak<RefCell<dyn NodeCoordinator>>>;

    fn get_aggregate_child_kind_set(&self) -> u32;

    fn set_aggregate_child_kind_set(&mut self, child_kind_set: u32);
}

#[Leak]
#[derive(Debug, Default, ModifierElement)]
pub(crate) struct ModifierNodeImpl {
    parent: Option<Weak<RefCell<dyn ModifierNode>>>,
    child: Option<Rc<RefCell<dyn ModifierNode>>>,
    coordinator: Option<Weak<RefCell<dyn NodeCoordinator>>>,
    aggregate_child_kind_set: u32,
}

impl NodeKindPatch for ModifierNodeImpl {
    fn get_node_kind(&self) -> NodeKind {
        todo!("implement get node kind by yourself")
    }
}

impl DelegatableNode for ModifierNodeImpl {
    fn get_node(&self) -> DelegatableKind {
        DelegatableKind::This
    }
}

impl ModifierNode for ModifierNodeImpl {
    fn set_parent(&mut self, parent: Option<Weak<RefCell<dyn ModifierNode>>>) {
        self.parent = parent;
    }

    fn get_parent(&self) -> Option<Rc<RefCell<dyn ModifierNode>>> {
        self.parent.try_upgrade()
    }

    fn set_child(&mut self, child: Option<Rc<RefCell<dyn ModifierNode>>>) {
        self.child = child
    }

    fn get_child(&self) -> Option<Rc<RefCell<dyn ModifierNode>>> {
        self.child.clone()
    }

    fn update_coordinator(&mut self, coordinator: Option<Weak<RefCell<dyn NodeCoordinator>>>) {
        self.coordinator = coordinator;
    }

    fn get_coordinator(&self) -> Option<Weak<RefCell<dyn NodeCoordinator>>> {
        self.coordinator.clone()
    }

    fn get_aggregate_child_kind_set(&self) -> u32 {
        self.aggregate_child_kind_set
    }

    fn set_aggregate_child_kind_set(&mut self, child_kind_set: u32) {
        self.aggregate_child_kind_set = child_kind_set;
    }
}

#[derive(Default)]
pub enum Modifier {
    #[default]
    Unit,
    ModifierNodeElement {
        create: Box<dyn FnMut() -> Rc<RefCell<dyn ModifierNode>>>,
        update: Box<dyn FnMut(RefMut<dyn ModifierNode>)>,
    },
    ModifierElement(
        Rc<RefCell<dyn ModifierElement>>
    ),
    Combined {
        left: Box<Modifier>,
        right: Box<Modifier>,
    },
}

#[inline]
pub(crate) fn modifier_node_element_creator<T>(mut creator: impl FnMut() -> T + 'static) -> Box<dyn FnMut() -> Rc<RefCell<dyn ModifierNode>>> where T: Sized + ModifierNode {
    (move || {
        creator().wrap_with_rc_refcell() as Rc<RefCell<dyn ModifierNode>>
    }).wrap_with_box()
}

#[inline]
pub(crate) fn modifier_node_element_updater<T>(mut updater: impl FnMut(&mut T) + 'static) -> Box<dyn FnMut(RefMut<dyn ModifierNode>)> where T: Sized + ModifierNode {
    (move |mut element: RefMut<dyn ModifierNode>| {
        if let Some(element) = element.as_any_mut().downcast_mut::<T>() {
            updater(element);
        } else {
            panic!("incorrect type for update modifier node element")
        }
    }).wrap_with_box()
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
                f.write_str("<modifier:node_element[")?;
                f.write_str(&format!("create:{:p}", create.deref()))?;
                f.write_str(&format!(",update:{:p}]>", update.deref()))
            }
            Modifier::ModifierElement(element) => {
                element.fmt(f)
            }
            _ => f.write_str("<unknown modifier>"),
        }
    }
}

pub(crate) trait ModifierNodeExtension {
    fn dispatch_for_kind(&self, kind: NodeKind, block: impl FnMut(&dyn ModifierElement));
    fn dispatch_for_kind_mut(&mut self, kind: NodeKind, block: impl FnMut(&mut dyn ModifierElement));
    fn next_draw_node(&self) -> Option<Rc<RefCell<dyn ModifierNode>>>;
    fn require_coordinator(&self, node_kind: NodeKind) -> Rc<RefCell<dyn NodeCoordinator>>;
}

impl<T> ModifierNodeExtension for T where T: ?Sized + ModifierNode {
    fn dispatch_for_kind(&self, kind: NodeKind, mut block: impl FnMut(&dyn ModifierElement)) {
        let node = self.get_node_kind();

        if node == kind {
            block(self.as_modifier_element());
        }
    }

    fn dispatch_for_kind_mut(&mut self, kind: NodeKind, mut block: impl FnMut(&mut dyn ModifierElement)) {
        let node = self.get_node_kind();

        if node == kind {
            block(self.as_modifier_element_mut());
        }
    }

    fn next_draw_node(&self) -> Option<Rc<RefCell<dyn ModifierNode>>> {
        let draw_mask = NodeKind::Draw.mask();
        let measure_mask = NodeKind::Layout.mask();

        let child = self.get_child();
        let mut next = child;

        while let Some(next_node) = next.clone() {
            let next_node_ref = next_node.borrow();
            let node_kind_set = next_node_ref.get_node_kind();
            if node_kind_set & measure_mask != 0 {
                return None;
            }
            if node_kind_set & draw_mask != 0 {
                return next;
            }

            next = next_node_ref.get_child();
        }

        None
    }

    fn require_coordinator(&self, node_kind: NodeKind) -> Rc<RefCell<dyn NodeCoordinator>> {
        let coordinator = self.get_coordinator().unwrap().upgrade().unwrap();
        let coordinator_ref = coordinator.borrow();
        if coordinator_ref.get_tail().as_ptr() as *const () != self as *const T as *const () {
            coordinator.clone()
        } else if node_kind.include_self_in_traversal() {
            coordinator_ref.get_wrapped().unwrap()
        } else {
            coordinator.clone()
        }
    }
}