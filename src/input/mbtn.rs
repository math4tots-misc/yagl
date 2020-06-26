use crate::winit;

/// A button on a mouse
///
/// Basically follows winit
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

impl MouseButton {
    pub(crate) fn from_winit(btn: winit::event::MouseButton) -> MouseButton {
        match btn {
            winit::event::MouseButton::Left => MouseButton::Left,
            winit::event::MouseButton::Right => MouseButton::Right,
            winit::event::MouseButton::Middle => MouseButton::Middle,
            winit::event::MouseButton::Other(x) => MouseButton::Other(x)
        }
    }
}
