use std::cell::RefCell;
use std::fmt::{Debug, Formatter, Write};
use std::ops::{Add, Deref};
use std::rc::Rc;
use crate::foundation::modifier::{Node, NodeImpl};

use super::modifier::Modifier;

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
            _ => {
                operation(initial, self)
            }
        }
    }

    pub fn fold_out<R>(&self, initial: R, operation: &mut dyn FnMut(&Modifier, R) -> R) -> R {
        match self {
            Modifier::Combined { left, right } => {
                left.fold_out(right.fold_out(initial, operation), operation)
            }
            _ => {
                operation(self, initial)
            }
        }
    }

    pub fn any(&self, mut predicate: impl FnMut(&Modifier) -> bool) -> bool {
        match self {
            Modifier::Combined { left, right } => {
                left.any(&mut predicate) || right.any(predicate)
            }
            _ => {
                predicate(self)
            }
        }
    }

    pub fn all(&self, mut predicate: impl FnMut(&Modifier) -> bool) -> bool {
        match self {
            Modifier::Combined { left, right } => {
                left.all(&mut predicate) && right.all(predicate)
            }
            _ => {
                predicate(self)
            }
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
            Modifier::ModifierNodeElement { .. } => { true }
            _ => { false }
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
            Modifier::Unit => {
                f.write_str("<modifier:unit>")
            }
            Modifier::Combined {
                left, right
            } => {
                f.write_str("<modifier:combined>")?;
                left.fmt(f)?;
                right.fmt(f)
            }
            Modifier::ModifierNodeElement {create, update} => {
                f.write_str("<modifier:element[")?;
                f.write_str(&format!("create:{:p}", create.deref()))?;
                f.write_str(&format!(",update:{:p}]>", update.deref()))
            }
            _ => {
                f.write_str("<unknown modifier>")
            }
        }
    }
}
