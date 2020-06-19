extern crate winit;
extern crate wgpu;
extern crate futures;
extern crate bytemuck;
extern crate image;
extern crate failure;
extern crate cgmath;
extern crate anyhow;
extern crate simple_logger;
extern crate log;
extern crate gfx_hal;

// specify the graphics backend
// For now, the choice is hardcoded based on OS
#[cfg(target_os = "windows")]
extern crate gfx_backend_dx12 as gfxb;
#[cfg(target_os = "macos")]
extern crate gfx_backend_metal as gfxb;
#[cfg(target_os = "linux")]
extern crate gfx_backend_vulkan as gfxb;

#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

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
pub use tut::tutorial_gfxhal_main;
pub use sandbox::s2d_main;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
