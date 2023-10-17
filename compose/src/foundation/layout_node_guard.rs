use std::marker::PhantomPinned;

pub struct LayoutNodeGuard<'a> {
    node: &'a LayoutNode,
    composer: &'a Composer,
    _data: PhantomPinned
}
