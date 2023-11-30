use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use crate::impl_node_kind_for_type;
use auto_delegate::Delegate;
use compose_foundation_macro::ModifierElement;
use crate::foundation::modifier::{Modifier, modifier_node_element_creator, modifier_node_element_updater, ModifierNodeImpl, NodeKind};
use crate::foundation::layout::layout_coordinates::LayoutCoordinates;
use crate::foundation::utils::box_wrapper::WrapWithBox;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;
use crate::impl_node_kind_any;
use std::rc::Rc;
use crate::foundation::geometry::IntSize;
use crate::foundation::modifier_node::LayoutAwareModifierNode;

impl Modifier {
    pub fn on_placed(self, on_place: impl Fn(&dyn LayoutCoordinates) + 'static) -> Modifier {
        self.then(on_placed_element(Rc::new(on_place)))
    }
}

#[derive(Delegate, ModifierElement)]
#[Impl(LayoutAwareModifierNodeConverter)]
struct OnPlacedNode {
    callback: Rc<dyn Fn(&dyn LayoutCoordinates)>,

    #[to(ModifierNode)]
    node_impl: ModifierNodeImpl,
}
impl_node_kind_for_type!(OnPlacedNode, NodeKind::LayoutAware);

impl LayoutAwareModifierNode for OnPlacedNode {
    fn on_placed(&self, coordinate: &dyn LayoutCoordinates) {
        (self.callback)(coordinate)
    }

    fn on_remeasured(&self, size: IntSize) {
        todo!()
    }
}

impl Debug for OnPlacedNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OnPlacedNode").field("node_impl", &self.node_impl).finish()
    }
}

impl OnPlacedNode {
    fn new(callback: Rc<dyn Fn(&dyn LayoutCoordinates)>) -> Self {
        Self {
            callback,
            node_impl: Default::default(),
        }
    }
}

fn on_placed_element(on_place: Rc<dyn Fn(&dyn LayoutCoordinates)>) -> Modifier {
    let on_place_for_update = on_place.clone();
    Modifier::ModifierNodeElement {
        create: modifier_node_element_creator(move || {
            OnPlacedNode::new(on_place.clone())
        }),
        update: modifier_node_element_updater(move |node: &mut OnPlacedNode| {
            node.callback = on_place_for_update.clone();
        }),
    }
}