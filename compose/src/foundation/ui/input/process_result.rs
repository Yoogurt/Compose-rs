#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) struct ProcessResult {
    value: i32,
}

impl ProcessResult {
    pub fn new(dispatched_to_A_pointer_input_modifier: bool, any_movement_consumed: bool) -> Self {
        Self {
            value: (dispatched_to_A_pointer_input_modifier as i32) | ((any_movement_consumed as i32) << 1)
        }
    }

    pub fn is_dispatched_to_a_pointer_input_modifier(&self) -> bool {
        (self.value & 1) != 0
    }

    pub fn is_any_movement_consumed(&self) -> bool {
        (self.value & (1 << 1)) != 0
    }
}