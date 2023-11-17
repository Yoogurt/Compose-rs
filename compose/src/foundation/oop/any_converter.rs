use std::any::Any;
use std::fmt::Debug;

pub trait AnyConverter: Any + Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
