use crate::AppContext;
use crate::GraphicsContext;
use crate::DeviceId;
use crate::Key;
use crate::anyhow::Result;

/// Trait describing the behavior of a game.
///
/// This is the main entry point in interacting with yagl.
///
/// To create a game with yagl, you just need to implement this trait
/// and return an instance of it in the closure you pass to
/// yagl::run.
#[allow(unused_variables)]
pub trait Game where Self: 'static + Sized {
    /// Called when the window is resized
    fn resize(&mut self, actx: &mut AppContext, width: u32, height: u32) -> Result<()> { Ok(()) }

    /// Called when drawing on the screen is requested
    /// TODO: In a perfect world, I actually want the signature to look like
    ///
    ///     fn draw(&self, actx: &mut AppContext) -> Result<Rc<impl Drawable>>
    ///
    /// alas, this is not yet possible in Rust because returning
    /// an impl trait from trait impls are blocked on higher kinded types.
    /// For now, ideally code implementing this method should look like:
    ///
    ///     fn draw(..) {
    ///         let drawable = ...
    ///         drawable.draw(actx, gctx)
    ///     }
    fn draw(&self, actx: &mut AppContext, gctx: &mut GraphicsContext) -> Result<()>;

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
