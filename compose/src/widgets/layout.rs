#[Compose]
pub fn layout(modifier: Modifier, measure_policy_delegate: MeasurePolicyDelegate, content: fn()) {
    let node = Composer::begin_node();
    node.borrow_mut().update(modifier, measure_policy_delegate);
    content();
    Composer::end_node(node);
}