use crate::gilrs;

/// A button on a gamepad
///
/// Follows the same model as gilrs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Button {
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

impl Button {
    pub(crate) fn from_gilrs(btn: gilrs::Button) -> Option<Button> {
        match btn {
            gilrs::Button::South => Some(Button::South),
            gilrs::Button::East => Some(Button::East),
            gilrs::Button::North => Some(Button::North),
            gilrs::Button::West => Some(Button::West),
            gilrs::Button::C => Some(Button::C),
            gilrs::Button::Z => Some(Button::Z),
            gilrs::Button::LeftTrigger => Some(Button::LeftTrigger),
            gilrs::Button::LeftTrigger2 => Some(Button::LeftTrigger2),
            gilrs::Button::RightTrigger => Some(Button::RightTrigger),
            gilrs::Button::RightTrigger2 => Some(Button::RightTrigger2),
            gilrs::Button::Select => Some(Button::Select),
            gilrs::Button::Start => Some(Button::Start),
            gilrs::Button::Mode => Some(Button::Mode),
            gilrs::Button::LeftThumb => Some(Button::LeftThumb),
            gilrs::Button::RightThumb => Some(Button::RightThumb),
            gilrs::Button::DPadUp => Some(Button::DPadUp),
            gilrs::Button::DPadDown => Some(Button::DPadDown),
            gilrs::Button::DPadLeft => Some(Button::DPadLeft),
            gilrs::Button::DPadRight => Some(Button::DPadRight),
            gilrs::Button::Unknown => Some(Button::Unknown),
        }
    }
}
