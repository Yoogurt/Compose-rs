use std::rc::Rc;
use crate::foundation::modifier::NodeKind;
use std::cell::RefCell;
use crate::foundation::ui::input::pointer_event::PointerEventPass;
use crate::foundation::delegatable_node::DelegatableNode;
use crate::foundation::geometry::IntSize;
use crate::foundation::layout::layout_coordinates::LayoutCoordinates;
use crate::foundation::modifier::{ModifierNode, ModifierNodeExtension};
use crate::foundation::ui::input::pointer_event::PointerEvent;

pub(crate) trait PointerInputModifierNode: ModifierNode + DelegatableNode {
    fn on_pointer_event(&self, event: &PointerEvent, pass: PointerEventPass, bounds: IntSize);

    fn on_cancel_pointer_input(&self);

    fn intercept_out_of_bounds_child_events(&self) -> bool {
        false
    }

    fn share_pointer_input_with_siblings(&self) -> bool {
        false
    }
}

pub(crate) trait PointerInputModifierNodeExtension {
    fn require_coordinator(&self, kind: NodeKind) -> Rc<RefCell<dyn LayoutCoordinates>>;
}

impl<T> PointerInputModifierNodeExtension for T where T: ?Sized + PointerInputModifierNode {
    fn require_coordinator(&self, kind: NodeKind) -> Rc<RefCell<dyn LayoutCoordinates>> {
        ModifierNodeExtension::require_coordinator(self, kind) as Rc<RefCell<dyn LayoutCoordinates>>
    }
}