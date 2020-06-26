use crate::gilrs;

/// A button on a gamepad
///
/// Follows the same model as gilrs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadButton {
    // Action Pad
    South,
    East,
    North,
    West,
    C,
    Z,
    // Triggers
    LeftTrigger,
    LeftTrigger2,
    RightTrigger,
    RightTrigger2,
    // Menu Pad
    Select,
    Start,
    Mode,
    // Sticks
    LeftThumb,
    RightThumb,
    // D-Pad
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,

    Unknown,
}

impl GamepadButton {
    pub(crate) fn from_gilrs(btn: gilrs::Button) -> Option<GamepadButton> {
        match btn {
            gilrs::Button::South => Some(GamepadButton::South),
            gilrs::Button::East => Some(GamepadButton::East),
            gilrs::Button::North => Some(GamepadButton::North),
            gilrs::Button::West => Some(GamepadButton::West),
            gilrs::Button::C => Some(GamepadButton::C),
            gilrs::Button::Z => Some(GamepadButton::Z),
            gilrs::Button::LeftTrigger => Some(GamepadButton::LeftTrigger),
            gilrs::Button::LeftTrigger2 => Some(GamepadButton::LeftTrigger2),
            gilrs::Button::RightTrigger => Some(GamepadButton::RightTrigger),
            gilrs::Button::RightTrigger2 => Some(GamepadButton::RightTrigger2),
            gilrs::Button::Select => Some(GamepadButton::Select),
            gilrs::Button::Start => Some(GamepadButton::Start),
            gilrs::Button::Mode => Some(GamepadButton::Mode),
            gilrs::Button::LeftThumb => Some(GamepadButton::LeftThumb),
            gilrs::Button::RightThumb => Some(GamepadButton::RightThumb),
            gilrs::Button::DPadUp => Some(GamepadButton::DPadUp),
            gilrs::Button::DPadDown => Some(GamepadButton::DPadDown),
            gilrs::Button::DPadLeft => Some(GamepadButton::DPadLeft),
            gilrs::Button::DPadRight => Some(GamepadButton::DPadRight),
            gilrs::Button::Unknown => Some(GamepadButton::Unknown),
        }
    }
}
