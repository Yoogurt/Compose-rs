use crate::foundation::ui::input::pointer_event::PointerEventType;
use std::collections::HashMap;
use crate::foundation::ui::input::pointer_button::{PointerButton, PointerButtons};
use crate::foundation::ui::input::pointer_event::{PointerId, PointerInputChange};
use crate::foundation::ui::input::pointer_event_type::PointerInputEvent;

pub(crate) struct InternalPointerEvent {
    pointer_event_type: PointerEventType,
    changes: HashMap<PointerId, PointerInputChange>,
    buttons: PointerButtons,
    button: Option<PointerButton>
}

impl InternalPointerEvent {
    pub(crate) fn new(changes: HashMap<PointerId, PointerInputChange>, pointer_input_event: PointerInputEvent) -> Self {
        Self {
            pointer_event_type: pointer_input_event.event_type,
            changes,
            buttons: pointer_input_event.buttons,
            button: pointer_input_event.button,
        }
    }

    pub(crate) fn pointer_event_type(&self) -> PointerEventType {
        self.pointer_event_type
    }

    pub(crate) fn changes(&self) -> &HashMap<PointerId, PointerInputChange> {
        &self.changes
    }
}