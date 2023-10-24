#[Compose]
pub fn layout(modifier: Modifier, measure_policy: MultiChildrenMeasurePolicy, content: fn()) {
    let node = Composer::begin_node();
    {
        let mut node_mut = node.borrow_mut();
        node_mut.set_measure_policy(measure_policy);
        node_mut.set_modifier(modifier);
    }
    content();
    Composer::end_node(node);
}