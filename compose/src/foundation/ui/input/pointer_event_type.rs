use crate::foundation::ui::input::internal_pointer_input::PointerInputEventData;
use crate::foundation::ui::input::pointer_event::PointerEventType;

pub(crate) struct PointerInputEvent {
    event_type: PointerEventType,
    uptime: u64,
    pointers: Vec<PointerInputEventData>,
    // buttons
}