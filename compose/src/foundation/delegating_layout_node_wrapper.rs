use super::{look_ahead_capable_placeable::NodeCoordinator, modifier::Modifier};

pub(crate) trait DelegatingLayoutNodeWrapper: NodeCoordinator {
    fn set_modifier_to(&mut self, modifier: Modifier);
}