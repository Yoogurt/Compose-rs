use crate::foundation::ui::input::pointer_event_type::PointerInputEvent;

pub(crate) trait GesstureOwner {
    fn process_pointer_input(&mut self, event: PointerInputEvent);
}