use crate::anyhow::Result;
use crate::AppContext;
use crate::DeviceId;
use crate::Key;
use crate::RenderContext;

/// Trait describing the behavior of a game.
///
/// This is the main entry point in interacting with yagl.
///
/// To create a game with yagl, you just need to implement this trait
/// and return an instance of it in the closure you pass to
/// yagl::run.
#[allow(unused_variables)]
pub trait Game
where
    Self: 'static + Sized,
{
    /// This method is called exactly once on start
    fn options(&self) -> Options {
        Options::default()
    }

    /// Called to check if the game should be updated
    fn update(&mut self, actx: &mut AppContext) -> Result<()>;

    /// Called when drawing on the screen is requested
    ///
    /// The RenderContext can retrieve the AppContext if needed with
    /// the `actx()` method.
    ///
    /// To render something on the screen, the `render` method
    /// on the RenderContext should be called exactly once.
    /// If called more than once, it may erase the previous draw
    fn render(&mut self, rctx: &mut RenderContext) -> Result<()>;

    /// Called when the window is resized
    fn resize(&mut self, actx: &mut AppContext, width: u32, height: u32) -> Result<()> {
        Ok(())
    }

    /// Called on character input
    fn ch(&mut self, actx: &mut AppContext, ch: char) -> Result<()> {
        Ok(())
    }

    /// Called to notify the game that a key was pressed.
    ///
    /// The default behavior of this method is to exit when Escape is pressed
    ///
    /// NOTE, not all keys may be recognized. If it isn't, this method
    /// will not get called for those keys.
    ///
    /// In the future, there should be a separate 'key_scancode_*' method
    /// so that even if the key is not recognized, the raw scancode can be
    /// passed to the client to process.
    fn key_pressed(&mut self, actx: &mut AppContext, dev: DeviceId, key: Key) -> Result<()> {
        if let Key::Escape = key {
            actx.exit();
        }
        Ok(())
    }

    /// Called to notify the game that a key was released.
    ///
    /// NOTE, not all keys may be recognized. If it isn't, this method
    /// will not get called for those keys.
    ///
    /// In the future, there should be a separate 'key_scancode_*' method
    /// so that even if the key is not recognized, the raw scancode can be
    /// passed to the client to process.
    fn key_released(&mut self, actx: &mut AppContext, dev: DeviceId, key: Key) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Options {
    /// Enable gamepad support
    ///
    /// This is currently not supported in winit directly,
    /// so there's a bit of overhead in doing so (i.e. a secondary
    /// thread is spawned).
    ///
    /// Enabled by default, but may be disabled if doing so is not desired
    pub enable_gamepad: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            enable_gamepad: true,
        }
    }
}
