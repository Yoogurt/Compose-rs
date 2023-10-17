pub struct LayoutNode {
    children: Vec<Box<LayoutNode>>,
    modifier: Modifier,
}