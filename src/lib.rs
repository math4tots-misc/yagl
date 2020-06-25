extern crate a2d;
extern crate anyhow;
extern crate futures;

use a2d::winit;

mod context;
mod game;
mod input;
mod window;

pub use context::AppContext;
pub use context::RenderContext;
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
pub use a2d::Rect;
pub use a2d::Point;
