use crate::gilrs;

/// An axis on a gamepad
///
/// Follows the same model as gilrs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    LeftStickX,
    LeftStickY,
    LeftZ,
    RightStickX,
    RightStickY,
    RightZ,
    DPadX,
    DPadY,
    Unknown,
}

impl Axis {
    pub(crate) fn from_gilrs(axis: gilrs::Axis) -> Axis {
        match axis {
            gilrs::Axis::LeftStickX => Axis::LeftStickX,
            gilrs::Axis::LeftStickY => Axis::LeftStickY,
            gilrs::Axis::LeftZ => Axis::LeftZ,
            gilrs::Axis::RightStickX => Axis::RightStickX,
            gilrs::Axis::RightStickY => Axis::RightStickY,
            gilrs::Axis::RightZ => Axis::RightZ,
            gilrs::Axis::DPadX => Axis::DPadX,
            gilrs::Axis::DPadY => Axis::DPadY,
            gilrs::Axis::Unknown => Axis::Unknown,
        }
    }
}
