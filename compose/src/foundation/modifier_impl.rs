use std::fmt::{Debug, Formatter};
use std::ops::Add;

use super::modifier::Modifier;

impl Modifier {
    pub fn then(self, modifier: Modifier) -> Modifier{
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
}

impl Add for Modifier {
    type Output = Modifier;
    fn add(self, rhs: Self) -> Self::Output {
       self.then(rhs)
    }
}

impl Debug for Modifier {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}