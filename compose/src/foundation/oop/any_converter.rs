use std::any::Any;

#[macro_export]
macro_rules! implement_any_by_self {
    ($tt:tt) => {
        impl crate::foundation::oop::AnyConverter for $tt {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };
}

pub trait AnyConverter: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
