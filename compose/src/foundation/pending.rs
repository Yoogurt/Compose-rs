use std::any::Any;
use crate::foundation::slot_table_type::GroupKind;

pub struct KeyInfo {
    key: u64,
    object_key: Option<Box<dyn Any>>,
}

pub struct Pending {
    key_info: KeyInfo
}