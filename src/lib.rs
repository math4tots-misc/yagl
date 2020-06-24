extern crate a2d;
extern crate anyhow;
extern crate bytemuck;
extern crate cgmath;
extern crate futures;
extern crate image;
extern crate log;
extern crate simple_logger;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use a2d::winit;

mod context;
mod game;
mod input;
mod window;

pub use context::AppContext;
pub use game::Game;
pub use input::DeviceId;
pub use input::Key;
pub use window::run;
pub use window::Window;

// re-exported from a2d
pub use a2d::Color;
pub use a2d::Dimensions;
pub use a2d::Instance;
pub use a2d::SpriteBatch;
pub use a2d::SpriteSheet;
pub use a2d::TextGrid;
