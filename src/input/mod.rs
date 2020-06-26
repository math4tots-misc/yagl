use crate::winit::event::DeviceId as WinitDeviceId;
use crate::gilrs::GamepadId;

mod axis;
mod key;
mod btn;

pub use axis::Axis;
pub use key::Key;
pub use btn::Button;

/// An ID that identifies an external device
/// Used for uniquely identifying the device that provided
/// a given input
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DeviceId(Enum);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Enum {
    Winit(WinitDeviceId),
    Gilrs(GamepadId),
}

impl From<WinitDeviceId> for DeviceId {
    fn from(d: WinitDeviceId) -> DeviceId {
        DeviceId(Enum::Winit(d))
    }
}

impl From<GamepadId> for DeviceId {
    fn from(d: GamepadId) -> DeviceId {
        DeviceId(Enum::Gilrs(d))
    }
}
