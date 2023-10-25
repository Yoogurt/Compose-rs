use super::{look_ahead_capable_placeable::LayoutNodeWrapper, modifier::Modifier};

pub(crate) trait DelegatingLayoutNodeWrapper: LayoutNodeWrapper {
    fn set_modifier_to(&mut self, modifier: Modifier);
}