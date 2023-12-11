use crate::foundation::ui::input::pointer_event_type::PointerInputEvent;
use crate::foundation::ui::input::process_result::ProcessResult;

pub(crate) trait GesstureOwner {
    fn process_pointer_input(&mut self, event: PointerInputEvent, is_in_bounds: bool) -> ProcessResult;
}