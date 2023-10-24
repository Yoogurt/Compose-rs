use skia_safe::op;
use crate::foundation::Modifier;
use std::ops::Add;

impl Modifier {
    pub fn fold_in<R>(self, initial: R, mut operation: impl FnMut(R, &Modifier) -> R) -> R {
        match self {
            Modifier::Combined { left, right } => {
                right.fold_in(left.fold_in(initial, &mut operation), operation)
            }
            _ => {
                operation(initial, &self)
            }
        }
    }

    pub fn fold_out<R>(self, initial: R, operation: &mut dyn FnMut(&Modifier, R) -> R) -> R {
        match self {
            Modifier::Combined { left, right } => {
                left.fold_out(right.fold_out(initial, operation), operation)
            }
            _ => {
                operation(&self, initial)
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
}

impl Add for Modifier {
    type Output = Modifier;
    fn add(self, rhs: Self) -> Self::Output {
        Modifier::Combined {
            left: Box::new(self),
            right: Box::new(rhs),
        }
    }
}