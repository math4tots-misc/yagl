use crate::winit::event::DeviceId as WinitDeviceId;

mod key;

/// An ID that identifies an external device
/// Used for uniquely identifying the device that provided
/// a given input
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceId(pub(crate) WinitDeviceId);

pub use key::Key;

