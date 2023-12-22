#![allow(non_upper_case_globals)]

use std::cell::{RefCell, RefMut};
use std::fmt::{Formatter, Write};
use std::fmt::Debug;
use std::ops::{Add, BitAnd, BitOr, Deref};
use std::rc::{Rc, Weak};

use auto_delegate::delegate;
use compose_foundation_macro::{Leak, ModifierElement};

use crate::foundation::delegatable_node::{DelegatableKind, DelegatableNode};
use crate::foundation::node_coordinator::NodeCoordinator;
use crate::foundation::oop::{AnyConverter, DrawModifierNodeConverter, LayoutAwareModifierNodeConverter, ParentDataModifierNodeConverter, PointerInputModifierNodeConverter};
use crate::foundation::oop::LayoutModifierNodeConverter;
use crate::foundation::utils::box_wrapper::WrapWithBox;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;
use crate::foundation::utils::weak_upgrade::WeakUpdater;

pub const Modifier: Modifier = Modifier {
    inner: ModifierInternal::Unit
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NodeKind {
    Any = 1 << 0,
    Layout = 1 << 1,
    ParentData = 1 << 2,
    Draw = 1 << 3,
    LayoutAware = 1 << 4,
    PointerInput = 1 << 5,
}

impl NodeKind {
    pub fn mask(&self) -> u32 {
        *self as u32
    }

    pub(crate) fn include_self_in_traversal(&self) -> bool {
        NodeKind::LayoutAware & self.mask() != 0
    }
}

pub(crate) fn calculate_node_kind_set_from(node: impl ModifierNode) {
    // if node.get_kind_set() {  }
}

pub(crate) fn calculate_node_kind_set_from_includeing_delegates(node: &Rc<RefCell<dyn ModifierNode>>) -> u32 {
    let node = node.borrow();

    match node.get_node() {
        DelegatableKind::This => {
            node.get_node_kind()
        }
        DelegatableKind::Other(other) => {
            todo!()
        }
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

impl BitOr<u32> for NodeKind {
    type Output = u32;
    fn bitor(self, rhs: u32) -> Self::Output {
        u32::from(self) | rhs
    }
}

#[macro_export]
macro_rules! impl_node_kind_any {
    ($tt:tt) => {
        impl NodeKindPatch for $tt {
            fn get_node_kind(&self) -> u32 {
                NodeKind::Any as u32
            }
        }
    };
}

pub trait NodeKindPatch {
    fn get_node_kind(&self) -> u32;

    fn is_node_kind(&self, node_kind: NodeKind) -> bool {
        (node_kind & self.get_node_kind()) != 0
    }
}

pub(crate) trait ModifierElement: AnyConverter + LayoutModifierNodeConverter + DrawModifierNodeConverter + ParentDataModifierNodeConverter + LayoutAwareModifierNodeConverter + PointerInputModifierNodeConverter + NodeKindPatch + Debug {
    fn as_modifier_element(&self) -> &dyn ModifierElement;
    fn as_modifier_element_mut(&mut self) -> &mut dyn ModifierElement;
}

#[delegate]
pub(crate) trait ModifierNode: ModifierElement + DelegatableNode {
    fn set_parent(&mut self, parent: Option<Weak<RefCell<dyn ModifierNode>>>);

    fn get_parent(&self) -> Option<Rc<RefCell<dyn ModifierNode>>>;

    fn set_child(&mut self, parent: Option<Rc<RefCell<dyn ModifierNode>>>);

    fn get_child(&self) -> Option<Rc<RefCell<dyn ModifierNode>>>;

    fn update_coordinator(&mut self, coordinator: Option<Weak<RefCell<dyn NodeCoordinator>>>);

    fn get_coordinator(&self) -> Option<Weak<RefCell<dyn NodeCoordinator>>>;

    fn get_aggregate_child_kind_set(&self) -> u32;

    fn set_aggregate_child_kind_set(&mut self, child_kind_set: u32);

    fn set_kind_set(&mut self, kind_set: u32);

    fn get_kind_set(&self) -> u32;

    fn is_attach(&self) -> bool;

    fn mark_as_attached(&mut self);

    fn mark_as_detached(&mut self);
}

#[Leak]
#[derive(Debug, Default, ModifierElement)]
pub(crate) struct ModifierNodeImpl {
    parent: Option<Weak<RefCell<dyn ModifierNode>>>,
    child: Option<Rc<RefCell<dyn ModifierNode>>>,
    coordinator: Option<Weak<RefCell<dyn NodeCoordinator>>>,
    aggregate_child_kind_set: u32,
    kind_set: u32,
    is_attach: bool
}

impl NodeKindPatch for ModifierNodeImpl {
    fn get_node_kind(&self) -> u32 {
        todo!("implement get node kind by yourself")
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

    fn get_kind_set(&self) -> u32 {
        self.kind_set
    }

    fn set_kind_set(&mut self, kind_set: u32) {
        self.kind_set = kind_set;
    }

    fn is_attach(&self) -> bool {
        self.is_attach
    }

    fn mark_as_attached(&mut self) {
        self.is_attach = true;
    }

    fn mark_as_detached(&mut self) {
        self.is_attach = false;
    }
}

#[derive(Default)]
pub struct Modifier {
    pub(crate) inner: ModifierInternal,
}

impl Debug for Modifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl From<ModifierInternal> for Modifier {
    fn from(value: ModifierInternal) -> Self {
        Modifier {
            inner: value
        }
    }
}

impl From<Modifier> for ModifierInternal {
    fn from(value: Modifier) -> Self {
        value.inner
    }
}

#[derive(Default, Clone)]
pub(crate) enum ModifierInternal {
    #[default]
    Unit,
    ModifierNodeElement {
        name: &'static str,
        create: Rc<dyn Fn() -> Rc<RefCell<dyn ModifierNode>>>,
        update: Rc<dyn Fn(RefMut<dyn ModifierNode>)>,
    },
    ComposedModifier {
        factory: Rc<dyn Fn(Modifier) -> Modifier>,
    },
    Combined {
        left: Box<ModifierInternal>,
        right: Box<ModifierInternal>,
    },
}

pub(crate) fn ModifierNodeElement<T>(name: &'static str, create: impl Fn() -> T + 'static, update: impl Fn(&mut T) + 'static) -> Modifier where T: Sized + ModifierNode {
    ModifierInternal::ModifierNodeElement {
        name,
        create: modifier_node_element_creator(create),
        update: modifier_node_element_updater(update),
    }.into()
}

#[inline]
fn modifier_node_element_creator<T>(creator: impl Fn() -> T + 'static) -> Rc<dyn Fn() -> Rc<RefCell<dyn ModifierNode>>> where T: Sized + ModifierNode {
    Rc::new(move || {
        creator().wrap_with_rc_refcell() as Rc<RefCell<dyn ModifierNode>>
    })
}

#[inline]
fn modifier_node_element_updater<T>(updater: impl Fn(&mut T) + 'static) -> Rc<dyn Fn(RefMut<dyn ModifierNode>)> where T: Sized + ModifierNode {
    Rc::new(move |mut element: RefMut<dyn ModifierNode>| {
        if let Some(element) = element.as_any_mut().downcast_mut::<T>() {
            updater(element);
        } else {
            panic!("incorrect type for update modifier node element")
        }
    })
}

impl ModifierInternal {
    fn fold_in<R>(self, initial: R, operation: &impl Fn(R, ModifierInternal) -> R) -> R {
        match self {
            ModifierInternal::Combined { left, right } => {
                right.fold_in(left.fold_in(initial, operation), operation)
            }
            _ => operation(initial, self),
        }
    }

    fn fold_out<R>(self, initial: R, operation: &impl Fn(ModifierInternal, R) -> R) -> R {
        match self {
            ModifierInternal::Combined { left, right } => {
                left.fold_out(right.fold_out(initial, operation), operation)
            }
            _ => operation(self, initial),
        }
    }

    fn any(&self, mut predicate: &impl Fn(&ModifierInternal) -> bool) -> bool {
        match self {
            ModifierInternal::Combined { left, right } => left.any(predicate) || right.any(predicate),
            _ => predicate(self),
        }
    }

    fn all(&self, mut predicate: &impl Fn(&ModifierInternal) -> bool) -> bool {
        match self {
            ModifierInternal::Combined { left, right } => left.all(predicate) && right.all(predicate),
            _ => predicate(self),
        }
    }
}

impl Modifier {
    fn all(&self, mut predicate: impl Fn(&ModifierInternal) -> bool) -> bool {
        self.inner.all(&predicate)
    }

    fn fold_in<R>(self, initial: R, operation: impl Fn(R, ModifierInternal) -> R) -> R {
        self.inner.fold_in(initial, &operation)
    }

    fn fold_out<R>(self, initial: R, operation: &impl Fn(ModifierInternal, R) -> R) -> R {
        self.inner.fold_out(initial, operation)
    }

    fn any(&self, mut predicate: impl Fn(&ModifierInternal) -> bool) -> bool {
        self.inner.any(&predicate)
    }

    pub fn then(mut self, modifier: Modifier) -> Modifier {
        if let ModifierInternal::Unit = self.inner {
            return modifier;
        }

        if let ModifierInternal::Unit = modifier.inner {
            return self;
        }

        self.inner = ModifierInternal::Combined {
            left: Box::new(self.inner),
            right: Box::new(modifier.inner),
        };

        self
    }

    pub fn composed(self, factory: impl Fn(Modifier) -> Modifier + 'static) -> Modifier {
        self.then(Modifier {
            inner: ModifierInternal::ComposedModifier {
                factory: Rc::new(factory),
            }
        })
    }

    pub fn materialize(self) -> Modifier {
        if self.all(|modifier| match modifier {
            ModifierInternal::ComposedModifier { .. } => { false }
            _ => { true }
        }) {
            return self;
        }

        self.fold_in(Modifier, |mut result, modifier| {
            match modifier {
                ModifierInternal::ComposedModifier { factory } => {
                    result = factory(result);
                }
                _ => {
                    result = result.then(modifier.clone().into());
                }
            }

            result
        })
    }

    pub(crate) fn flatten(self) -> Vec<Modifier> {
        let mut result = Vec::<Modifier>::with_capacity(16);
        let mut stack: Vec<Modifier> = vec![self];

        while let Some(next) = stack.pop() {
            match next.inner {
                ModifierInternal::Combined { left, right } => {
                    stack.push((*right).into());
                    stack.push((*left).into());
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
        match self.inner {
            ModifierInternal::ModifierNodeElement { .. } => true,
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

impl Debug for ModifierInternal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModifierInternal::Unit => f.write_str("<modifier:unit>"),
            ModifierInternal::Combined { left, right } => {
                f.write_str("<modifier:combined>")?;
                left.fmt(f)?;
                right.fmt(f)
            }
            ModifierInternal::ModifierNodeElement { name, create, update } => {
                f.write_str(&format!("<modifier:{}[", name))?;
                f.write_str(&format!("create:{:p}", create.deref()))?;
                f.write_str(&format!(",update:{:p}]>", update.deref()))
            }
            _ => f.write_str("<unknown modifier>"),
        }
    }
}

pub(crate) trait ModifierNodeExtension {
    fn visit_local_descendants(&self, mask: u32, block: impl FnMut(&dyn ModifierElement));
    fn dispatch_for_kind(&self, kind: NodeKind, block: impl FnMut(&dyn ModifierElement));
    fn dispatch_for_kind_mut(&mut self, kind: NodeKind, block: impl FnMut(&mut dyn ModifierElement));
    fn next_draw_node(&self) -> Option<Rc<RefCell<dyn ModifierNode>>>;
    fn require_coordinator(&self, node_kind: NodeKind) -> Rc<RefCell<dyn NodeCoordinator>>;
}

impl<T> ModifierNodeExtension for T where T: ?Sized + ModifierNode {
    fn visit_local_descendants(&self, mask: u32, mut block: impl FnMut(&dyn ModifierElement)) {
        let aggregate_child_kind_set = self.get_aggregate_child_kind_set();
        if aggregate_child_kind_set & mask == 0 {
            return;
        }

        let mut next=  self.get_child();
        while let Some(node) = next {
            let node = node.borrow();
            if node.get_node_kind() & mask != 0 {
                block(node.deref());
            }

            next = node.get_child();
        }
    }

    fn dispatch_for_kind(&self, kind: NodeKind, mut block: impl FnMut(&dyn ModifierElement)) {
        let node = self.get_node_kind();

        if (kind & node) != 0 {
            block(self.as_modifier_element());
        }
    }

    fn dispatch_for_kind_mut(&mut self, kind: NodeKind, mut block: impl FnMut(&mut dyn ModifierElement)) {
        let node = self.get_node_kind();

        if (kind & node) != 0 {
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