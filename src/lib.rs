extern crate winit;
extern crate wgpu;
extern crate futures;
extern crate bytemuck;
extern crate image;
extern crate failure;
extern crate cgmath;
extern crate anyhow;

mod tut;
mod sandbox;
mod window;
mod graphics;
mod game;
mod context;
mod input;

pub use game::Game;
pub use window::Window;
pub use window::run;
pub use graphics::Graphics;
pub use graphics::Drawable;
pub use graphics::DrawTask;
pub use graphics::TestDrawable;
pub use graphics::GraphicsContext;
pub use context::AppContext;
pub use input::DeviceId;
pub use input::Key;

use graphics::shaders;
use graphics::GraphicsGlobals;
use context::Globals;

pub use tut::tutorial_wgpu_main;
pub use sandbox::s2d_main;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
