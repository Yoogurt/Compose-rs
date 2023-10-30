use std::cell::RefCell;
use std::rc::Weak;
use core::any::Any;
use std::fmt::Debug;

pub trait DelegatableNode : Debug{
    // fn get_node(&self) -> Weak<RefCell<dyn Node>>;
}