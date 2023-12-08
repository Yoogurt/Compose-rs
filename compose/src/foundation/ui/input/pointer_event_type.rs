use crate::foundation::geometry::Offset;
use crate::foundation::ui::input::internal_pointer_input::PointerInputEventData;
use crate::foundation::ui::input::pointer_button::{PointerButton, PointerButtons};
use crate::foundation::ui::input::pointer_event::PointerEventType;

#[derive(Clone, Debug)]
pub(crate) struct PointerInputEvent {
    pub(crate) event_type: PointerEventType,
    pub(crate) time_millis: u128,
    pub(crate) pointers: Vec<PointerInputEventData>,
    pub(crate) buttons: PointerButtons,
    pub(crate) button: Option<PointerButton>,
    pub(crate) scroll_delta: Offset<f32>,
}

impl Default for PointerInputEvent {
    fn default() -> Self {
        Self {
            event_type: PointerEventType::Unknown,
            time_millis: 0,
            pointers: vec![],
            buttons: PointerButtons::default(),
            button: None,
            scroll_delta: Offset::zero(),
        }
    }
}

impl PointerInputEvent {
    pub fn new(event_type: PointerEventType,
               time_millis: u128,
               pointers: Vec<PointerInputEventData>,
               buttons: PointerButtons,
               button: Option<PointerButton>,
               scroll_delta: Offset<f32>,
    ) -> Self {
        Self {
            event_type,
            time_millis,
            pointers,
            buttons,
            button,
            scroll_delta,
        }
    }

    pub fn is_gesture_in_progress(&self) -> bool {
        self.pointers.iter().any(|pointer| pointer.down)
    }
}