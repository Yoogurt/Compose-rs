pub(crate) trait DelegatingLayoutNodeWrapper: LayoutNodeWrapper {
    fn set_modifier_to(&mut self, modifier: Modifier);
}