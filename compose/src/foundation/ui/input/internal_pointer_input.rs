use crate::foundation::geometry::Offset;

pub(crate) struct PointerInputEventData {
    id: u64,
    uptime: u64,
    position_on_screen: Offset<f32>,
    position: Offset<f32>,
    down: bool,
    pressure: f32,

    scroll_delta: Offset<f32>,
}