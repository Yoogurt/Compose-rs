#[derive(Clone, PartialEq, Debug)]
pub(crate) enum PointerButton {
    Primary = 0,
    Secondary = 1,
    Tertiary = 2,
    Back = 3,
    Forward = 4,
}

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct PointerButtons {
    packed_value: i32,
}

pub(crate) struct PointerButtonsPressed {
    is_primary_pressed: bool,
    is_secondary_pressed: bool,
    is_tertiary_pressed: bool,
    is_back_pressed: bool,
    is_forward_pressed: bool,
}

impl Default for PointerButtonsPressed {
    fn default() -> Self {
        Self {
            is_primary_pressed: false,
            is_secondary_pressed: false,
            is_tertiary_pressed: false,
            is_back_pressed: false,
            is_forward_pressed: false,
        }
    }
}

impl PointerButtons {
    pub fn new(pointer_pressed: PointerButtonsPressed) -> Self {
        let mut packed_value = 0;
        if pointer_pressed.is_primary_pressed {
            packed_value |= 1 << PointerButton::Primary as i32;
        }
        if pointer_pressed.is_secondary_pressed {
            packed_value |= 1 << PointerButton::Secondary as i32;
        }
        if pointer_pressed.is_tertiary_pressed {
            packed_value |= 1 << PointerButton::Tertiary as i32;
        }
        if pointer_pressed.is_back_pressed {
            packed_value |= 1 << PointerButton::Back as i32;
        }
        if pointer_pressed.is_forward_pressed {
            packed_value |= 1 << PointerButton::Forward as i32;
        }

        Self {
            packed_value
        }
    }

    pub(crate) fn copy_for(&self, button: PointerButton, pressed: bool) -> Self {
        let mut button_pressed = PointerButtonsPressed {
            is_primary_pressed: self.primary(),
            is_secondary_pressed: self.secondary(),
            is_tertiary_pressed: self.tertiary(),
            is_back_pressed: self.back(),
            is_forward_pressed: self.forward(),
        };

        match button {
            PointerButton::Primary => button_pressed.is_primary_pressed = pressed,
            PointerButton::Secondary => button_pressed.is_secondary_pressed = pressed,
            PointerButton::Tertiary => button_pressed.is_tertiary_pressed = pressed,
            PointerButton::Back => button_pressed.is_back_pressed = pressed,
            PointerButton::Forward => button_pressed.is_forward_pressed = pressed,
        }

        Self::new(button_pressed)
    }

    pub fn primary(&self) -> bool {
        self.packed_value & (1 << PointerButton::Primary as i32) != 0
    }

    pub fn secondary(&self) -> bool {
        self.packed_value & (1 << PointerButton::Secondary as i32) != 0
    }

    pub fn tertiary(&self) -> bool {
        self.packed_value & (1 << PointerButton::Tertiary as i32) != 0
    }

    pub fn back(&self) -> bool {
        self.packed_value & (1 << PointerButton::Back as i32) != 0
    }

    pub fn forward(&self) -> bool {
        self.packed_value & (1 << PointerButton::Forward as i32) != 0
    }

    pub fn are_any_pressed(&self) -> bool {
        self.packed_value != 0
    }
}