use crate::foundation::geometry::Offset;
use crate::foundation::ui::input::pointer_event::{HistoricalChange, PointerId, PointerType};

#[derive(Clone, Debug)]
pub(crate) struct PointerInputEventData {
    pub(crate) id: PointerId,
    pub(crate) uptime: u128,
    pub(crate) position_on_screen: Offset<f32>,
    pub(crate) position: Offset<f32>,
    pub(crate) down: bool,
    pub(crate) pressure: f32,
    pub(crate) pointer_type: PointerType,

    pub(crate) histroical: Vec<HistoricalChange>,
    pub(crate) scroll_delta: Offset<f32>,
}