use crate::foundation::ui::input::pointer_event::PointerEventPass;
use crate::foundation::delegatable_node::DelegatableNode;
use crate::foundation::geometry::IntSize;
use crate::foundation::ui::input::pointer_event::PointerEvent;

pub(crate) trait PointerInputModifierNode: DelegatableNode {
    fn on_pointer_event(&self, event: &PointerEvent, pass: PointerEventPass, bounds: IntSize);

    fn on_cancel_pointer_input(&self);

    fn intercept_out_of_bounds_child_events(&self) -> bool {
        false
    }

    fn share_pointer_input_with_siblings(&self) -> bool {
        false
    }
}