use crate::foundation::geometry::Offset;

#[derive(Clone, PartialEq, Debug, Copy)]
pub(crate) enum PointerEventType {
    Unknown = 0,
    Press = 1,
    Release = 2,
    Move = 3,
    Enter = 4,
    Exit = 5,
    Scroll = 6,
}

#[derive(Clone, PartialEq, Debug, Copy)]
pub(crate) enum PointerType {
    Unknown = 0,
    Touch = 1,
    Mouse = 2,
    Stylus = 3,
    Eraser = 4,
}

#[derive(Clone, PartialEq, Eq, Debug, Copy, Hash)]
pub(crate) struct PointerId {
    value: u64,
}

impl PointerId {
    pub(crate) fn new(value: u64) -> Self {
        Self {
            value
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct HistoricalChange {
    uptime: u128,
    position: Offset<f32>,
}

pub(crate) struct ConsumedData {
    position_change: bool,
    down_change: bool,
}

impl ConsumedData {
    pub(crate) fn new(position_change: bool, down_change: bool) -> Self {
        Self {
            position_change,
            down_change,
        }
    }
}

pub(crate) struct PointerInputChange {
    pub(crate) id: PointerId,
    pub(crate) uptime: u128,
    pub(crate) position: Offset<f32>,
    pub(crate) pressed: bool,
    pub(crate) pressure: f32,
    pub(crate) previous_up_time: u128,
    pub(crate) previous_position: Offset<f32>,
    pub(crate) previous_pressed: bool,
    pub(crate) is_initially_consumed: bool,
    pub(crate) pointer_type: PointerType,
    pub(crate) scroll_delta: Offset<f32>,

    historical: Vec<HistoricalChange>,
    consumed_data: ConsumedData,
}

impl PointerInputChange {
    pub(crate) fn new(id: PointerId,
                      uptime: u128,
                      position: Offset<f32>,
                      pressed: bool,
                      pressure: f32,
                      previous_up_time: u128,
                      previous_position: Offset<f32>,
                      previous_pressed: bool,
                      is_initially_consumed: bool,
                      pointer_type: PointerType,
                      historical: Vec<HistoricalChange>,
                      scroll_delta: Offset<f32>) -> Self {
        Self {
            id,
            uptime,
            position,
            pressed,
            pressure,
            previous_up_time,
            previous_position,
            previous_pressed,
            is_initially_consumed,
            pointer_type,
            scroll_delta,

            historical,
            consumed_data: ConsumedData::new(is_initially_consumed, is_initially_consumed),
        }
    }

    pub(crate) fn is_consumed(&self) -> bool {
        self.consumed_data.down_change || self.consumed_data.position_change
    }

    pub(crate) fn consume(&mut self) {
        self.consumed_data.down_change = true;
        self.consumed_data.position_change = true;
    }

    pub(crate) fn changed_to_down_ignore_consumed(&self) -> bool {
        !self.previous_pressed && self.pressed
    }
}